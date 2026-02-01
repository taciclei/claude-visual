//! Claude API types

use serde::{Deserialize, Serialize};

/// Claude message format
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ClaudeMessage {
    pub(crate) role: String,
    pub(crate) content: String,
}

/// Claude request
#[derive(Debug, Serialize)]
pub(crate) struct ClaudeRequest {
    pub(crate) model: String,
    pub(crate) messages: Vec<ClaudeMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) system: Option<String>,
    pub(crate) max_tokens: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stop_sequences: Option<Vec<String>>,
    pub(crate) stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tools: Option<Vec<ClaudeTool>>,
}

/// Claude tool format
#[derive(Debug, Serialize)]
pub(crate) struct ClaudeTool {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) input_schema: serde_json::Value,
}

/// Claude response
#[derive(Debug, Deserialize)]
pub(crate) struct ClaudeResponse {
    pub(crate) id: String,
    pub(crate) model: String,
    pub(crate) content: Vec<ContentBlock>,
    pub(crate) stop_reason: Option<String>,
    pub(crate) usage: ClaudeUsage,
}

/// Content block
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub(crate) enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
}

/// Usage stats
#[derive(Debug, Deserialize)]
pub(crate) struct ClaudeUsage {
    pub(crate) input_tokens: usize,
    pub(crate) output_tokens: usize,
}

/// Stream event
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub(crate) enum StreamEvent {
    #[serde(rename = "content_block_start")]
    ContentBlockStart {
        index: usize,
        content_block: ContentBlock,
    },
    #[serde(rename = "content_block_delta")]
    ContentBlockDelta { index: usize, delta: Delta },
    #[serde(rename = "content_block_stop")]
    ContentBlockStop { index: usize },
    #[serde(rename = "message_start")]
    MessageStart { message: MessageMeta },
    #[serde(rename = "message_delta")]
    MessageDelta { delta: MessageDeltaContent },
    #[serde(rename = "message_stop")]
    MessageStop,
    #[serde(other)]
    Other,
}

/// Delta content
#[derive(Debug, Deserialize)]
pub(crate) struct Delta {
    #[serde(rename = "type")]
    pub(crate) delta_type: Option<String>,
    pub(crate) text: Option<String>,
}

/// Message metadata
#[derive(Debug, Deserialize)]
pub(crate) struct MessageMeta {
    pub(crate) id: String,
    pub(crate) model: String,
}

/// Message delta content
#[derive(Debug, Deserialize)]
pub(crate) struct MessageDeltaContent {
    pub(crate) stop_reason: Option<String>,
}
