//! AI Provider Trait
//!
//! Defines the common interface for AI providers.

mod error;
mod message;
mod model;
mod request;
mod response;
mod stream;
mod provider_trait;
mod config;

pub use error::AIError;
pub use message::{Message, MessageRole};
pub use model::ModelInfo;
pub use request::{AIRequest, ToolDefinition};
pub use response::{AIResponse, StopReason, ToolCall, Usage};
pub use stream::{AIStream, StreamChunk};
pub use provider_trait::AIProvider;
pub use config::ProviderConfig;
