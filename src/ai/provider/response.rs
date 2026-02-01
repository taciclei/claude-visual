//! AI response types

use serde::{Deserialize, Serialize};

/// Tool call from the model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Tool call ID
    pub id: String,
    /// Tool name
    pub name: String,
    /// Tool arguments as JSON
    pub arguments: serde_json::Value,
}

/// AI response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    /// Response ID
    pub id: String,
    /// Model used
    pub model: String,
    /// Generated content
    pub content: String,
    /// Stop reason
    pub stop_reason: Option<StopReason>,
    /// Tool calls
    pub tool_calls: Vec<ToolCall>,
    /// Usage statistics
    pub usage: Option<Usage>,
}

/// Stop reason
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StopReason {
    /// End of turn (natural completion)
    EndTurn,
    /// Stop sequence hit
    StopSequence,
    /// Max tokens reached
    MaxTokens,
    /// Tool use requested
    ToolUse,
}

/// Token usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    /// Input tokens
    pub input_tokens: usize,
    /// Output tokens
    pub output_tokens: usize,
    /// Total tokens
    pub total_tokens: usize,
}
