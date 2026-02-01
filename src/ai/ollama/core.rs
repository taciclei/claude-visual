//! Ollama provider core implementation

use super::types::*;
use crate::ai::provider::{AIError, AIRequest, ModelInfo, ProviderConfig};

/// Default Ollama API URL
const OLLAMA_API_URL: &str = "http://localhost:11434";

/// Ollama provider for local models
pub struct OllamaProvider {
    /// HTTP client
    pub(crate) client: reqwest::Client,
    /// Configuration
    pub(crate) config: ProviderConfig,
    /// Cached model list
    pub(crate) cached_models: Vec<ModelInfo>,
}

impl OllamaProvider {
    /// Create a new Ollama provider
    pub fn new(config: ProviderConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(
                config.timeout_secs.unwrap_or(300), // Longer timeout for local models
            ))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            config,
            cached_models: Vec::new(),
        }
    }

    /// Create with default settings
    pub fn local() -> Self {
        Self::new(ProviderConfig::default())
    }

    /// Create with custom base URL
    pub fn with_url(url: impl Into<String>) -> Self {
        Self::new(ProviderConfig {
            base_url: Some(url.into()),
            ..Default::default()
        })
    }

    /// Get API base URL
    pub(crate) fn base_url(&self) -> &str {
        self.config.base_url.as_deref().unwrap_or(OLLAMA_API_URL)
    }

    /// Fetch available models from Ollama
    pub async fn refresh_models(&mut self) -> Result<(), AIError> {
        let response = self
            .client
            .get(format!("{}/api/tags", self.base_url()))
            .send()
            .await
            .map_err(|e| AIError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AIError::Provider(format!(
                "Failed to fetch models: {}",
                response.status()
            )));
        }

        let tags: OllamaTagsResponse = response
            .json()
            .await
            .map_err(|e| AIError::Provider(format!("Invalid response: {}", e)))?;

        self.cached_models = tags
            .models
            .into_iter()
            .map(|m| ModelInfo {
                id: m.name.clone(),
                name: m.name.clone(),
                provider: "Ollama".to_string(),
                context_length: self.estimate_context_length(&m.name),
                supports_streaming: true,
                supports_tools: self.model_supports_tools(&m.name),
                supports_vision: self.model_supports_vision(&m.name),
                input_cost_per_1k: None, // Local models are free
                output_cost_per_1k: None,
            })
            .collect();

        Ok(())
    }

    /// Estimate context length based on model name
    pub(crate) fn estimate_context_length(&self, model_name: &str) -> usize {
        // Try to parse context length from model name or use defaults
        if model_name.contains("128k") || model_name.contains("128000") {
            128_000
        } else if model_name.contains("32k") || model_name.contains("32000") {
            32_000
        } else if model_name.contains("16k") || model_name.contains("16000") {
            16_000
        } else if model_name.contains("llama3") || model_name.contains("llama-3") {
            8_192
        } else if model_name.contains("mistral") {
            32_000
        } else if model_name.contains("qwen") {
            32_000
        } else if model_name.contains("phi") {
            2_048
        } else if model_name.contains("gemma") {
            8_192
        } else {
            4_096 // Default
        }
    }

    /// Check if model supports tool use
    pub(crate) fn model_supports_tools(&self, model_name: &str) -> bool {
        // Most modern models support tools
        model_name.contains("llama3")
            || model_name.contains("mistral")
            || model_name.contains("qwen")
            || model_name.contains("command")
    }

    /// Check if model supports vision
    pub(crate) fn model_supports_vision(&self, model_name: &str) -> bool {
        model_name.contains("llava")
            || model_name.contains("bakllava")
            || model_name.contains("vision")
            || model_name.contains("moondream")
    }

    /// Convert internal request to Ollama API format
    pub(crate) fn build_api_request(&self, request: &AIRequest) -> OllamaGenerateRequest {
        let mut prompt = String::new();

        // Build prompt from messages
        if let Some(system) = &request.system {
            prompt.push_str(&format!("System: {}\n\n", system));
        }

        for msg in &request.messages {
            let role = match msg.role {
                crate::ai::provider::MessageRole::System => "System",
                crate::ai::provider::MessageRole::User => "User",
                crate::ai::provider::MessageRole::Assistant => "Assistant",
                crate::ai::provider::MessageRole::Tool => "Tool",
            };
            prompt.push_str(&format!("{}: {}\n\n", role, msg.content));
        }

        prompt.push_str("Assistant: ");

        OllamaGenerateRequest {
            model: request.model.clone(),
            prompt,
            stream: request.stream,
            options: Some(OllamaOptions {
                temperature: request.temperature,
                top_p: request.top_p,
                num_predict: request.max_tokens.map(|n| n as i32),
                stop: request.stop.clone(),
            }),
        }
    }

    /// Build chat request (for chat-optimized models)
    pub(crate) fn build_chat_request(&self, request: &AIRequest) -> OllamaChatRequest {
        let mut messages: Vec<OllamaMessage> = Vec::new();

        // Add system message
        if let Some(system) = &request.system {
            messages.push(OllamaMessage {
                role: "system".to_string(),
                content: system.clone(),
            });
        }

        // Add conversation messages
        for msg in &request.messages {
            messages.push(OllamaMessage {
                role: match msg.role {
                    crate::ai::provider::MessageRole::System => "system".to_string(),
                    crate::ai::provider::MessageRole::User => "user".to_string(),
                    crate::ai::provider::MessageRole::Assistant => "assistant".to_string(),
                    crate::ai::provider::MessageRole::Tool => "tool".to_string(),
                },
                content: msg.content.clone(),
            });
        }

        OllamaChatRequest {
            model: request.model.clone(),
            messages,
            stream: request.stream,
            options: Some(OllamaOptions {
                temperature: request.temperature,
                top_p: request.top_p,
                num_predict: request.max_tokens.map(|n| n as i32),
                stop: request.stop.clone(),
            }),
        }
    }
}
