//! Chat Input Completion Integration
//!
//! Provides LSP-powered auto-completion for code snippets in chat input,
//! file path completion, and @mention completion.

mod types;
mod manager;
mod completions;
mod navigation;
mod utils;

#[cfg(test)]
mod tests;

// Re-export public API
pub use types::{
    CompletionTrigger,
    ChatCompletionItem,
    ChatCompletionKind,
    CompletionConfig,
    CompletionState,
    CompletionResult,
};

pub use manager::InputCompletionManager;
