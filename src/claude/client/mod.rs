//! Claude Code process management

mod core;
mod parser;
mod stream;

pub use core::ClaudeClient;

/// Options for sending a prompt to Claude
#[derive(Debug, Clone, Default)]
pub struct PromptOptions {
    /// Enable extended thinking mode
    pub think_mode: bool,
    /// Model to use (e.g., "claude-sonnet-4-20250514", "claude-opus-4-20250514")
    pub model: Option<String>,
    /// Session ID for continuing a conversation
    pub session_id: Option<String>,
}
