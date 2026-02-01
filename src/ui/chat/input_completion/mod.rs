//! Chat Input Completion Integration
//!
//! Provides LSP-powered auto-completion for code snippets in chat input,
//! file path completion, and @mention completion.

mod completions;
mod manager;
mod navigation;
mod types;
mod utils;

#[cfg(test)]
mod tests;

// Re-export public API
pub use types::{
    ChatCompletionItem, ChatCompletionKind, CompletionConfig, CompletionResult, CompletionState,
    CompletionTrigger,
};

pub use manager::InputCompletionManager;
