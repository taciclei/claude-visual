//! AI request types and tool definitions

use super::message::Message;
use serde::{Deserialize, Serialize};

/// AI request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRequest {
    /// Model to use
    pub model: String,
    /// Conversation messages
    pub messages: Vec<Message>,
    /// System prompt (optional, prepended to messages)
    pub system: Option<String>,
    /// Maximum tokens to generate
    pub max_tokens: Option<usize>,
    /// Temperature (0.0 - 2.0)
    pub temperature: Option<f64>,
    /// Top-p sampling
    pub top_p: Option<f64>,
    /// Stop sequences
    pub stop: Option<Vec<String>>,
    /// Whether to stream the response
    pub stream: bool,
    /// Available tools
    pub tools: Option<Vec<ToolDefinition>>,
}

impl Default for AIRequest {
    fn default() -> Self {
        Self {
            model: String::new(),
            messages: Vec::new(),
            system: None,
            max_tokens: None,
            temperature: None,
            top_p: None,
            stop: None,
            stream: true,
            tools: None,
        }
    }
}

/// Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Tool name
    pub name: String,
    /// Tool description
    pub description: String,
    /// JSON schema for parameters
    pub parameters: serde_json::Value,
}
