/// Per-turn context mode. Ask turns keep the model request lean; act turns
/// keep the current full tool/MCP/skill injection path.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContextMode {
    Ask,
    Act,
}

impl ContextMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ask => "ask",
            Self::Act => "act",
        }
    }

    pub fn is_ask(self) -> bool {
        matches!(self, Self::Ask)
    }
}

/// Classify a user prompt before assembling the model request.
///
/// Ask: greetings, explanations, status, scoring, and other conversation that
/// should not preload Skill bodies or the full tool/MCP schema.
/// Act: an action verb plus an explicit path, command, or repository target.
pub fn classify_context_mode(prompt: &str) -> ContextMode {
    let trimmed = prompt.trim();
    if trimmed.is_empty() {
        return ContextMode::Ask;
    }
    if looks_like_act(trimmed) {
        ContextMode::Act
    } else {
        ContextMode::Ask
    }
}

fn looks_like_act(original: &str) -> bool {
    let lower = original.to_ascii_lowercase();
    has_action_verb(&lower, original) && has_explicit_target(&lower, original)
}

fn has_action_verb(lower: &str, original: &str) -> bool {
    const VERBS: &[&str] = &[
        "read_file",
        "write_file",
        "edit_file",
        "apply_patch",
        "local_shell",
        "list_skills",
        "read_skill",
        "git clone",
        "git push",
        "git commit",
        "npm run",
        "cargo test",
        "cargo build",
        "改文件",
        "改代码",
        "写代码",
        "写文件",
        "跑命令",
        "执行命令",
        "克隆",
        "提交代码",
        "推送",
        "编译",
        "构建",
        "打开",
        "读取",
        "保存到",
        "删除",
        "备份",
        "安装",
        "卸载",
        "调试",
        "重构",
        "clone",
        "commit",
        "push",
        "install",
        "build",
        "run ",
        "edit ",
        "write ",
        "read ",
        "open ",
        "fix ",
        "patch ",
    ];
    VERBS
        .iter()
        .any(|verb| lower.contains(verb) || original.contains(verb))
}

fn has_explicit_target(lower: &str, original: &str) -> bool {
    const TARGETS: &[&str] = &[
        "/workspace",
        "/home/coomi",
        "~/custom_coomi",
        "custom_coomi",
        "read_file",
        "write_file",
        "edit_file",
        "apply_patch",
        "local_shell",
        "list_skills",
        "read_skill",
        "git clone",
        "git push",
        "git commit",
        "npm run",
        "cargo test",
        "cargo build",
    ];
    if TARGETS
        .iter()
        .any(|target| lower.contains(target) || original.contains(target))
    {
        return true;
    }
    original.contains("./")
        || original.contains("~/")
        || original
            .split_whitespace()
            .any(|token| token.starts_with('/') && token.len() > 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greetings_and_status_are_ask() {
        assert_eq!(classify_context_mode("你好"), ContextMode::Ask);
        assert_eq!(classify_context_mode("这个应用还差什么功能"), ContextMode::Ask);
        assert_eq!(classify_context_mode("你还有好建议吗？"), ContextMode::Ask);
        assert_eq!(classify_context_mode("看一下评分"), ContextMode::Ask);
        assert_eq!(classify_context_mode("这个技能怎么样"), ContextMode::Ask);
    }

    #[test]
    fn url_only_questions_are_ask() {
        assert_eq!(
            classify_context_mode("https://github.com/TensorHub-ORG/Coomi 这个是新版吗"),
            ContextMode::Ask
        );
        assert_eq!(
            classify_context_mode("README.md 和 settings.json 有什么区别"),
            ContextMode::Ask
        );
        assert_eq!(
            classify_context_mode("用 `gh` 是什么意思"),
            ContextMode::Ask
        );
    }

    #[test]
    fn verb_plus_explicit_path_is_act() {
        assert_eq!(
            classify_context_mode("按这个改代码，先改 /workspace/finance-v2/server.js"),
            ContextMode::Act
        );
        assert_eq!(
            classify_context_mode("clone TensorHub-ORG/Coomi into ~/custom_coomi"),
            ContextMode::Act
        );
        assert_eq!(
            classify_context_mode("打开 /workspace/finance-v2/HANDOFF.md"),
            ContextMode::Act
        );
        assert_eq!(classify_context_mode("跑 npm run preflight"), ContextMode::Act);
        assert_eq!(
            classify_context_mode("读取 /home/coomi/custom_coomi/README.md"),
            ContextMode::Act
        );
    }
}
