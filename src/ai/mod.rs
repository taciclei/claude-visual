//! AI Module
//!
//! Multi-model AI provider abstraction supporting Claude, OpenAI, and local models.

pub mod claude;
pub mod context;
pub mod mention;
pub mod ollama;
pub mod openai;
pub mod provider;
pub mod summarizer;

pub use context::{ContextItem, ContextItemType, ContextManager};
pub use mention::{get_mention_at_cursor, parse_mentions, Mention, MentionKind, PartialMention};
pub use provider::{AIError, AIProvider, AIRequest, AIResponse, Message, MessageRole, StreamChunk};
pub use summarizer::{
    ConversationMessage, ConversationSummary, MessageRole as SummaryMessageRole,
    SummarizationConfig, SummarizationRequest, SummarizationStats, Summarizer,
};
