//! Conversation Summarization
//!
//! Provides automatic summarization for long conversations to manage context windows.

mod analysis;
mod summarizer;
mod types;
mod utils;

#[cfg(test)]
mod tests;

// Re-export public types and main struct
pub use summarizer::Summarizer;
pub use types::{
    ConversationMessage, ConversationSummary, MessageRole, SummarizationConfig,
    SummarizationRequest, SummarizationStats,
};
