//! AI Provider Trait
//!
//! Defines the common interface for AI providers.

mod config;
mod error;
mod message;
mod model;
mod provider_trait;
mod request;
mod response;
mod stream;

pub use config::ProviderConfig;
pub use error::AIError;
pub use message::{Message, MessageRole};
pub use model::ModelInfo;
pub use provider_trait::AIProvider;
pub use request::{AIRequest, ToolDefinition};
pub use response::{AIResponse, StopReason, ToolCall, Usage};
pub use stream::{AIStream, StreamChunk};
