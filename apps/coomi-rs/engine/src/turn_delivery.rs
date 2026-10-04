//! Engine-owned durable outbox. Hook acknowledgements never enter model context.
use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use uuid::Uuid;

static OUTBOX_LOCK: Mutex<()> = Mutex::new(());

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PendingTurn {
    pub session_id: Uuid,
    pub turn_id: Uuid,
    pub content_sha256: String,
    pub payload: Value,
}

impl PendingTurn {
    pub fn new(payload: Value) -> Result<Self> {
        let session_id = payload["session_id"].as_str().context("missing session id")?.parse()?;
        let turn_id = payload["turn_id"].as_str().context("missing turn id")?.parse()?;
        // Hash an explicit ordered array; identical UTF-8 serialization in Rust and JS.
        let content = serde_json::json!([
            payload["session_id"], payload["turn_id"], payload["user_message_id"],
            payload["final_message_id"], payload["user_text"], payload["assistant_text"],
            payload["status"]
        ]);
        let content_sha256 = format!("{:x}", Sha256::digest(serde_json::to_vec(&content)?));
        Ok(Self { session_id, turn_id, content_sha256, payload })
    }

    pub fn delivery_payload(&self) -> Value {
        let mut payload = self.payload.clone();
        payload["content_sha256"] = Value::String(self.content_sha256.clone());
        payload
    }

    pub fn accepts_ack(&self, result: &Value) -> bool {
        let ack = &result["archive_ack"];
        ack["session_id"].as_str() == Some(self.session_id.to_string().as_str())
            && ack["turn_id"].as_str() == Some(self.turn_id.to_string().as_str())
            && ack["content_sha256"].as_str() == Some(self.content_sha256.as_str())
            && ack["status"].as_str() == Some("persisted")
    }
}

pub struct TurnDeliveryQueue {
    directory: PathBuf,
}

impl TurnDeliveryQueue {
    pub fn new(engine_home: impl AsRef<Path>) -> Self {
        Self { directory: engine_home.as_ref().join("turn-delivery") }
    }

    fn path(&self, turn: &PendingTurn) -> PathBuf {
        self.directory.join(format!("{}-{}.json", turn.session_id, turn.turn_id))
    }

    /// Durable before dispatch. Same key with changed content is never overwritten.
    pub fn enqueue(&self, turn: &PendingTurn) -> Result<()> {
        let _guard = OUTBOX_LOCK.lock().map_err(|_| anyhow::anyhow!("outbox lock poisoned"))?;
        fs::create_dir_all(&self.directory)?;
        #[cfg(unix)] {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&self.directory, fs::Permissions::from_mode(0o700))?;
        }
        let target = self.path(turn);
        if target.exists() {
            let old: PendingTurn = serde_json::from_slice(&fs::read(&target)?)?;
            ensure!(old == *turn, "turn delivery conflict");
            return Ok(());
        }
        let temporary = self.directory.join(format!(".{}.tmp", Uuid::new_v4()));
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)] {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let result = (|| -> Result<()> {
            let mut file = options.open(&temporary)?;
            file.write_all(&serde_json::to_vec(turn)?)?;
            file.sync_all()?;
            fs::rename(&temporary, &target)?;
            File::open(&self.directory)?.sync_all()?;
            Ok(())
        })();
        if result.is_err() { let _ = fs::remove_file(&temporary); }
        result
    }

    pub fn pending(&self, session_id: Uuid) -> Result<Vec<PendingTurn>> {
        let _guard = OUTBOX_LOCK.lock().map_err(|_| anyhow::anyhow!("outbox lock poisoned"))?;
        if !self.directory.exists() { return Ok(Vec::new()); }
        let prefix = format!("{session_id}-");
        let mut paths = fs::read_dir(&self.directory)?
            .map(|entry| entry.map(|entry| entry.path()))
            .collect::<std::io::Result<Vec<_>>>()?;
        paths.sort();
        let mut turns = Vec::new();
        for path in paths {
            let name = path.file_name().and_then(|name| name.to_str()).unwrap_or("");
            if !name.starts_with(&prefix) || !name.ends_with(".json") { continue; }
            let turn: PendingTurn = serde_json::from_slice(&fs::read(&path)?)?;
            ensure!(turn.session_id == session_id && self.path(&turn) == path, "outbox identity mismatch");
            ensure!(PendingTurn::new(turn.payload.clone())? == turn, "outbox hash mismatch");
            turns.push(turn);
        }
        Ok(turns)
    }

    /// Only a matching durable archive receipt can remove an outstanding event.
    pub fn acknowledge(&self, turn: &PendingTurn, result: &Value) -> Result<bool> {
        if !turn.accepts_ack(result) { return Ok(false); }
        let _guard = OUTBOX_LOCK.lock().map_err(|_| anyhow::anyhow!("outbox lock poisoned"))?;
        let target = self.path(turn);
        if !target.exists() { return Ok(true); }
        let old: PendingTurn = serde_json::from_slice(&fs::read(&target)?)?;
        ensure!(old == *turn, "turn delivery conflict");
        fs::remove_file(target)?;
        File::open(&self.directory)?.sync_all()?;
        Ok(true)
    }

    // ── 2026-10-04：重放次数上限（T11 已知项治本）────────────────
    // 背景：replay_turn_delivery 每轮开头重放全部 pending，无上限无节流。
    // 必然失败的轮次（如 recovery-turn）永远拿不到 ack，会无限堆积并拖慢首 token。
    // 方案：用独立计数目录记录每个 turn 的重放次数，不污染 PendingTurn
    //       （其 PartialEq/哈希校验只认 payload 的 7 个字段，加字段会导致
    //        pending() 的 outbox hash mismatch 校验失败）。
    // 超限后：事件移入 permanently-failed/，队列不再重放它，数据不丢、可追溯。

    /// 计数目录；与事件文件分离，避免污染 PendingTurn 结构。
    fn attempts_dir(&self) -> PathBuf {
        self.directory.join(".attempts")
    }

    /// 已废弃事件目录；超限事件移入此处，保留原文供人工排查。
    pub fn failed_dir(&self) -> PathBuf {
        self.directory.join("permanently-failed")
    }

    fn attempt_file(&self, turn: &PendingTurn) -> PathBuf {
        self.attempts_dir()
            .join(format!("{}-{}", turn.session_id, turn.turn_id))
    }

    /// 记录一次重放尝试，返回累计次数（从 1 开始）。
    pub fn note_attempt(&self, turn: &PendingTurn) -> Result<u32> {
        let _guard = OUTBOX_LOCK.lock().map_err(|_| anyhow::anyhow!("outbox lock poisoned"))?;
        fs::create_dir_all(self.attempts_dir())?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(self.attempts_dir(), fs::Permissions::from_mode(0o700))?;
        }
        let path = self.attempt_file(turn);
        let current: u32 = match fs::read_to_string(&path) {
            Ok(text) => text.trim().parse().unwrap_or(0),
            Err(_) => 0,
        };
        let next = current.saturating_add(1);
        let mut file = File::create(&path)?;
        file.write_all(next.to_string().as_bytes())?;
        file.sync_all()?;
        Ok(next)
    }

    /// 归档成功后清除计数（该轮不再需要重放）。
    pub fn clear_attempts(&self, turn: &PendingTurn) -> Result<()> {
        let _guard = OUTBOX_LOCK.lock().map_err(|_| anyhow::anyhow!("outbox lock poisoned"))?;
        let path = self.attempt_file(turn);
        if path.exists() { fs::remove_file(path)?; }
        Ok(())
    }

    /// 重放超限：把事件移入 permanently-failed/，并从队列移除。
    /// 数据不删除（保留原文与 payload），仅停止无限重放。
    pub fn retire(&self, turn: &PendingTurn) -> Result<()> {
        let _guard = OUTBOX_LOCK.lock().map_err(|_| anyhow::anyhow!("outbox lock poisoned"))?;
        let target = self.path(turn);
        if target.exists() {
            fs::create_dir_all(self.failed_dir())?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(self.failed_dir(), fs::Permissions::from_mode(0o700))?;
            }
            let dest = self.failed_dir().join(
                target.file_name().ok_or_else(|| anyhow::anyhow!("bad outbox path"))?,
            );
            fs::rename(&target, &dest)?;
            File::open(&self.directory)?.sync_all()?;
        }
        let attempt = self.attempt_file(turn);
        if attempt.exists() { fs::remove_file(attempt)?; }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restart_replay_conflict_and_receipt_validation() {
        let home = tempfile::tempdir().unwrap();
        let payload = serde_json::json!({
            "session_id": Uuid::new_v4().to_string(), "turn_id": Uuid::new_v4().to_string(),
            "user_message_id": "u1", "final_message_id": "a1",
            "user_text": "问题", "assistant_text": "答复", "status": "success"
        });
        let turn = PendingTurn::new(payload).unwrap();
        let queue = TurnDeliveryQueue::new(home.path());
        queue.enqueue(&turn).unwrap();
        queue.enqueue(&turn).unwrap();
        let restored = TurnDeliveryQueue::new(home.path());
        assert_eq!(restored.pending(turn.session_id).unwrap(), vec![turn.clone()]);
        assert!(restored.pending(Uuid::new_v4()).unwrap().is_empty());
        let mut changed = turn.payload.clone();
        changed["assistant_text"] = Value::String("不同答复".into());
        assert!(restored.enqueue(&PendingTurn::new(changed).unwrap()).is_err());
        assert!(!restored.acknowledge(&turn, &serde_json::json!({"acked": true})).unwrap());
        let receipt = serde_json::json!({"archive_ack": {
            "session_id": turn.session_id.to_string(), "turn_id": turn.turn_id.to_string(),
            "content_sha256": turn.content_sha256, "status": "persisted"
        }});
        let mut wrong = receipt.clone();
        wrong["archive_ack"]["content_sha256"] = Value::String("wrong".into());
        assert!(!restored.acknowledge(&turn, &wrong).unwrap());
        assert!(restored.acknowledge(&turn, &receipt).unwrap());
        assert!(restored.pending(turn.session_id).unwrap().is_empty());
    }
}
