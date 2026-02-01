//! Core ClaudeClient implementation

use std::process::{Command, Stdio};

/// Client for interacting with Claude Code CLI
#[derive(Clone)]
pub struct ClaudeClient {
    /// Path to the claude CLI (defaults to "claude")
    pub(crate) cli_path: String,
}

impl Default for ClaudeClient {
    fn default() -> Self {
        Self::new()
    }
}

impl ClaudeClient {
    /// Create a new Claude client
    pub fn new() -> Self {
        Self {
            cli_path: "claude".to_string(),
        }
    }

    /// Create a client with a custom CLI path
    pub fn with_cli_path(cli_path: impl Into<String>) -> Self {
        Self {
            cli_path: cli_path.into(),
        }
    }

    /// Check if the Claude CLI is available
    pub fn check_available(&self) -> bool {
        Command::new(&self.cli_path)
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
}
