//! AI Module
//!
//! Multi-model AI provider abstraction supporting Claude, OpenAI, and local models.

pub mod provider;
pub mod claude;
pub mod openai;
pub mod ollama;
pub mod context;
pub mod mention;
pub mod summarizer;

pub use provider::{AIProvider, AIRequest, AIResponse, AIError, Message, MessageRole, StreamChunk};
pub use context::{ContextManager, ContextItem, ContextItemType};
pub use mention::{parse_mentions, get_mention_at_cursor, Mention, MentionKind, PartialMention};
pub use summarizer::{
    Summarizer, SummarizationConfig, SummarizationStats, SummarizationRequest,
    ConversationMessage, ConversationSummary, MessageRole as SummaryMessageRole,
};
