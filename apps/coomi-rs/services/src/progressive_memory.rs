use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

const DEFAULT_MAX_RECALL_ENTRIES: usize = 5;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProgressiveAppendResult {
    pub dialogue_id: String,
    pub turn_number: usize,
    pub summary_due: bool,
    pub duplicate: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct TurnEntry {
    kind: String,
    turn_id: String,
    user: String,
    assistant: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SummaryRequest {
    kind: String,
    dialogue_id: String,
    turn_number: usize,
    turn_range: (usize, usize),
}

/// Durable per-session progressive memory owned by the Coomi host.
///
/// It deliberately writes only under `<home>/progressive-memory`, never reads
/// arbitrary user paths, and keeps raw dialogue append-only. A stable turn id
/// makes retries and hook replays idempotent.
#[derive(Clone, Debug)]
pub struct ProgressiveMemoryStore {
    dialogue_id: String,
    log_path: PathBuf,
    summary_queue_path: PathBuf,
    summary_every: usize,
    inject_budget_chars: usize,
}

impl ProgressiveMemoryStore {
    pub fn open(
        home: &Path,
        dialogue_id: &str,
        summary_every: usize,
        inject_budget_chars: usize,
    ) -> Result<Self> {
        validate_dialogue_id(dialogue_id)?;
        anyhow::ensure!(summary_every > 0, "summary_every must be positive");
        anyhow::ensure!(inject_budget_chars > 0, "inject budget must be positive");
        let root = home.join("progressive-memory").join(dialogue_id);
        fs::create_dir_all(&root)
            .with_context(|| format!("failed to create progressive memory root {}", root.display()))?;
        Ok(Self {
            dialogue_id: dialogue_id.to_owned(),
            log_path: root.join("conversation_log.jsonl"),
            summary_queue_path: root.join("summary_requests.jsonl"),
            summary_every,
            inject_budget_chars,
        })
    }

    pub fn append_turn(
        &self,
        turn_id: &str,
        user: &str,
        assistant: &str,
    ) -> Result<ProgressiveAppendResult> {
        anyhow::ensure!(!turn_id.trim().is_empty(), "turn_id must not be empty");
        let entries = self.entries()?;
        if entries.iter().any(|entry| entry.turn_id == turn_id) {
            return Ok(ProgressiveAppendResult {
                dialogue_id: self.dialogue_id.clone(),
                turn_number: entries.len(),
                summary_due: entries.len() % self.summary_every == 0,
                duplicate: true,
            });
        }
        let entry = TurnEntry {
            kind: "raw_turn".into(),
            turn_id: turn_id.to_owned(),
            user: user.to_owned(),
            assistant: assistant.to_owned(),
        };
        append_json_line(&self.log_path, &entry)?;
        let turn_number = entries.len().saturating_add(1);
        let summary_due = turn_number % self.summary_every == 0;
        if summary_due {
            append_json_line(
                &self.summary_queue_path,
                &SummaryRequest {
                    kind: "summary_request".into(),
                    dialogue_id: self.dialogue_id.clone(),
                    turn_number,
                    turn_range: (
                        turn_number.saturating_sub(self.summary_every).saturating_add(1),
                        turn_number,
                    ),
                },
            )?;
        }
        Ok(ProgressiveAppendResult {
            dialogue_id: self.dialogue_id.clone(),
            turn_number,
            summary_due,
            duplicate: false,
        })
    }

    pub fn context(&self, query: &str) -> Result<String> {
        let query_terms = tokenize(query);
        if query_terms.is_empty() {
            return Ok(String::new());
        }
        let mut candidates = self
            .entries()?
            .into_iter()
            .enumerate()
            .filter_map(|(index, entry)| {
                let corpus = format!("{}\n{}", entry.user, entry.assistant).to_lowercase();
                let score = query_terms.iter().filter(|term| corpus.contains(*term)).count();
                (score > 0).then_some((score, index, entry))
            })
            .collect::<Vec<_>>();
        candidates.sort_by(|left, right| right.0.cmp(&left.0).then_with(|| right.1.cmp(&left.1)));

        let mut output = String::new();
        for (_, _, entry) in candidates.into_iter().take(DEFAULT_MAX_RECALL_ENTRIES) {
            let item = format!("user: {}\nassistant: {}\n", entry.user, entry.assistant);
            if output.len().saturating_add(item.len()) > self.inject_budget_chars {
                let remaining = self.inject_budget_chars.saturating_sub(output.len());
                if remaining > 0 {
                    output.push_str(&clip_to_boundary(&item, remaining));
                }
                break;
            }
            output.push_str(&item);
        }
        Ok(output)
    }

    pub fn raw_turn_count(&self) -> Result<usize> {
        Ok(self.entries()?.len())
    }

    pub fn pending_summary_count(&self) -> Result<usize> {
        Ok(read_json_lines::<SummaryRequest>(&self.summary_queue_path)?.len())
    }

    fn entries(&self) -> Result<Vec<TurnEntry>> {
        read_json_lines(&self.log_path)
    }
}

fn validate_dialogue_id(value: &str) -> Result<()> {
    anyhow::ensure!(
        !value.is_empty()
            && value.len() <= 128
            && value
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_'),
        "invalid dialogue_id"
    );
    Ok(())
}

fn append_json_line<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let rendered = serde_json::to_string(value)?;
    use std::io::Write;
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("failed to append {}", path.display()))?;
    file.write_all(rendered.as_bytes())?;
    file.write_all(b"\n")?;
    file.sync_data()?;
    Ok(())
}

fn read_json_lines<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<Vec<T>> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error).with_context(|| format!("failed to read {}", path.display())),
    };
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(line).context("invalid progressive memory JSONL entry"))
        .collect()
}

fn tokenize(text: &str) -> Vec<String> {
    let mut terms = text
        .split(|character: char| !character.is_alphanumeric())
        .map(str::trim)
        .filter(|term| term.chars().count() >= 2)
        .map(str::to_lowercase)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if terms.is_empty() {
        let compact = text.trim().to_lowercase();
        if compact.chars().count() >= 2 {
            terms.push(compact);
        }
    }
    terms
}

fn clip_to_boundary(value: &str, limit: usize) -> String {
    if value.len() <= limit {
        return value.to_owned();
    }
    let mut end = limit;
    while end > 0 && !value.is_char_boundary(end) {
        end = end.saturating_sub(1);
    }
    value[..end].to_owned()
}
