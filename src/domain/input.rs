//! Input types for Claude Code status line

use std::path::PathBuf;

use serde::Deserialize;

/// Input JSON from Claude Code status line invocation
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ClaudeInput {
    /// Working directory path (optional, falls back to cwd)
    #[serde(default)]
    pub cwd: Option<PathBuf>,

    /// Current model information
    #[serde(default)]
    pub model: Option<ModelInfo>,

    /// Context window usage (optional)
    #[serde(default)]
    pub context_window: Option<ContextWindow>,

    /// Path to current session transcript file
    #[serde(default)]
    pub transcript_path: Option<PathBuf>,

    /// Session size in bytes (calculated from transcript file)
    #[serde(default)]
    pub session_size_bytes: Option<u64>,
}

impl ClaudeInput {
    /// Get the project name from cwd or current directory
    pub fn project_name(&self) -> String {
        self.cwd
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                std::env::current_dir()
                    .ok()
                    .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
                    .unwrap_or_else(|| "unknown".to_string())
            })
    }

    /// Get the model from input or default
    pub fn get_model(&self) -> Model {
        self.model
            .as_ref()
            .map(|m| Model::from_display_name(&m.display_name))
            .unwrap_or_else(|| Model::from_display_name("Unknown"))
    }
}

/// Model information from input
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ModelInfo {
    pub display_name: String,
}

/// Context window usage information
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ContextWindow {
    pub current_usage: Option<ContextUsage>,
    pub context_window_size: Option<u64>,
}

/// Context usage breakdown
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ContextUsage {
    pub input_tokens: u64,
    pub cache_creation_input_tokens: u64,
    pub cache_read_input_tokens: u64,
}

/// Claude model identifier - stores the original display name from Claude Code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Model {
    display_name: String,
}

impl Model {
    /// Create from display name string
    pub fn from_display_name(name: &str) -> Self {
        Self {
            display_name: name.to_string(),
        }
    }

    /// Display name for status line (returns the original name from Claude Code)
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_from_display_name() {
        let model = Model::from_display_name("Opus 4.5");
        assert_eq!(model.display_name(), "Opus 4.5");

        let model = Model::from_display_name("Sonnet 4");
        assert_eq!(model.display_name(), "Sonnet 4");

        let model = Model::from_display_name("Haiku");
        assert_eq!(model.display_name(), "Haiku");
    }

    #[test]
    fn test_claude_input_deserialize_empty() {
        let json = "{}";
        let input: ClaudeInput = serde_json::from_str(json).unwrap();
        assert!(input.cwd.is_none());
        assert!(input.model.is_none());
        assert!(input.transcript_path.is_none());
    }

    #[test]
    fn test_claude_input_deserialize_full() {
        let json = r#"{
            "cwd": "/Users/dev/my-project",
            "model": {"display_name": "Opus 4.5"},
            "transcript_path": "/Users/dev/.claude/session.jsonl"
        }"#;
        let input: ClaudeInput = serde_json::from_str(json).unwrap();
        assert_eq!(
            input.cwd.as_ref().unwrap().to_str().unwrap(),
            "/Users/dev/my-project"
        );
        assert_eq!(input.model.as_ref().unwrap().display_name, "Opus 4.5");
        assert!(input.transcript_path.is_some());
    }

    #[test]
    fn test_claude_input_project_name() {
        let json = r#"{"cwd": "/Users/dev/my-project"}"#;
        let input: ClaudeInput = serde_json::from_str(json).unwrap();
        assert_eq!(input.project_name(), "my-project");
    }

    #[test]
    fn test_claude_input_get_model() {
        let json = r#"{"model": {"display_name": "Sonnet 4"}}"#;
        let input: ClaudeInput = serde_json::from_str(json).unwrap();
        assert_eq!(input.get_model().display_name(), "Sonnet 4");
    }

    #[test]
    fn test_claude_input_get_model_default() {
        let json = "{}";
        let input: ClaudeInput = serde_json::from_str(json).unwrap();
        assert_eq!(input.get_model().display_name(), "Unknown");
    }
}
