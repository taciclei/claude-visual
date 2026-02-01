//! OpenAI API types for request/response serialization

use serde::{Deserialize, Serialize};

/// OpenAI API request
#[derive(Debug, Serialize)]
pub(crate) struct OpenAIApiRequest {
    pub(crate) model: String,
    pub(crate) messages: Vec<OpenAIMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tools: Option<Vec<OpenAITool>>,
}

/// OpenAI message
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct OpenAIMessage {
    pub(crate) role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tool_calls: Option<Vec<OpenAIToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tool_call_id: Option<String>,
}

/// OpenAI tool
#[derive(Debug, Serialize)]
pub(crate) struct OpenAITool {
    #[serde(rename = "type")]
    pub(crate) tool_type: String,
    pub(crate) function: OpenAIFunction,
}

/// OpenAI function
#[derive(Debug, Serialize)]
pub(crate) struct OpenAIFunction {
    pub(crate) name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,
    pub(crate) parameters: serde_json::Value,
}

/// OpenAI tool call
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct OpenAIToolCall {
    pub(crate) id: String,
    #[serde(rename = "type")]
    pub(crate) call_type: String,
    pub(crate) function: OpenAIFunctionCall,
}

/// OpenAI function call
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct OpenAIFunctionCall {
    pub(crate) name: String,
    pub(crate) arguments: String,
}

/// OpenAI API response
#[derive(Debug, Deserialize)]
pub(crate) struct OpenAIApiResponse {
    pub(crate) id: String,
    pub(crate) model: String,
    pub(crate) choices: Vec<OpenAIChoice>,
    pub(crate) usage: Option<OpenAIUsage>,
}

/// OpenAI choice
#[derive(Debug, Deserialize)]
pub(crate) struct OpenAIChoice {
    pub(crate) message: OpenAIMessage,
    pub(crate) finish_reason: Option<String>,
}

/// OpenAI usage
#[derive(Debug, Deserialize)]
pub(crate) struct OpenAIUsage {
    pub(crate) prompt_tokens: usize,
    pub(crate) completion_tokens: usize,
    pub(crate) total_tokens: usize,
}

/// OpenAI stream event
#[derive(Debug, Deserialize)]
pub(crate) struct OpenAIStreamEvent {
    pub(crate) choices: Vec<OpenAIStreamChoice>,
}

/// OpenAI stream choice
#[derive(Debug, Deserialize)]
pub(crate) struct OpenAIStreamChoice {
    pub(crate) delta: OpenAIDelta,
    pub(crate) finish_reason: Option<String>,
}

/// OpenAI delta
#[derive(Debug, Deserialize)]
pub(crate) struct OpenAIDelta {
    pub(crate) content: Option<String>,
}
