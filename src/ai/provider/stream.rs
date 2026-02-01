//! Streaming response types

use super::error::AIError;
use super::response::{StopReason, Usage};
use futures::Stream;
use std::pin::Pin;

/// Streaming chunk
#[derive(Debug, Clone)]
pub enum StreamChunk {
    /// Text content
    Text(String),
    /// Tool call start
    ToolCallStart { id: String, name: String },
    /// Tool call argument delta
    ToolCallDelta { id: String, arguments: String },
    /// Stop event
    Stop(StopReason),
    /// Usage statistics
    Usage(Usage),
    /// Error
    Error(String),
}

/// Stream type for responses
pub type AIStream = Pin<Box<dyn Stream<Item = Result<StreamChunk, AIError>> + Send>>;
