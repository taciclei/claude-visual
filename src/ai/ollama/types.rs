//! Ollama API types

use serde::{Deserialize, Serialize};

/// Ollama tags response
#[derive(Debug, Deserialize)]
pub(crate) struct OllamaTagsResponse {
    pub(crate) models: Vec<OllamaModel>,
}

/// Ollama model info
#[derive(Debug, Deserialize)]
pub(crate) struct OllamaModel {
    pub(crate) name: String,
    #[allow(dead_code)]
    pub(crate) size: Option<u64>,
}

/// Ollama generate request (legacy)
#[derive(Debug, Serialize)]
pub(crate) struct OllamaGenerateRequest {
    pub(crate) model: String,
    pub(crate) prompt: String,
    pub(crate) stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) options: Option<OllamaOptions>,
}

/// Ollama chat request
#[derive(Debug, Serialize)]
pub(crate) struct OllamaChatRequest {
    pub(crate) model: String,
    pub(crate) messages: Vec<OllamaMessage>,
    pub(crate) stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) options: Option<OllamaOptions>,
}

/// Ollama message
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct OllamaMessage {
    pub(crate) role: String,
    pub(crate) content: String,
}

/// Ollama options
#[derive(Debug, Serialize)]
pub(crate) struct OllamaOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) num_predict: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stop: Option<Vec<String>>,
}

/// Ollama chat response
#[derive(Debug, Deserialize)]
pub(crate) struct OllamaChatResponse {
    pub(crate) model: String,
    pub(crate) message: OllamaMessage,
    pub(crate) done: bool,
    pub(crate) prompt_eval_count: Option<usize>,
    pub(crate) eval_count: Option<usize>,
}

/// Ollama stream event
#[derive(Debug, Deserialize)]
pub(crate) struct OllamaStreamEvent {
    #[allow(dead_code)]
    pub(crate) model: Option<String>,
    pub(crate) message: Option<OllamaMessage>,
    pub(crate) done: bool,
}
