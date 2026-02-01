//! AIProvider trait implementation for Ollama

use async_trait::async_trait;
use futures::StreamExt;

use super::core::OllamaProvider;
use super::types::{OllamaChatResponse, OllamaStreamEvent};
use crate::ai::provider::{
    AIError, AIProvider, AIRequest, AIResponse, AIStream, ModelInfo, StopReason, StreamChunk, Usage,
};

#[async_trait]
impl AIProvider for OllamaProvider {
    fn name(&self) -> &str {
        "Ollama"
    }

    fn models(&self) -> Vec<ModelInfo> {
        if self.cached_models.is_empty() {
            // Return some common models as defaults
            vec![
                ModelInfo {
                    id: "llama3.2".to_string(),
                    name: "Llama 3.2".to_string(),
                    provider: "Ollama".to_string(),
                    context_length: 128_000,
                    supports_streaming: true,
                    supports_tools: true,
                    supports_vision: false,
                    input_cost_per_1k: None,
                    output_cost_per_1k: None,
                },
                ModelInfo {
                    id: "llama3.1".to_string(),
                    name: "Llama 3.1".to_string(),
                    provider: "Ollama".to_string(),
                    context_length: 128_000,
                    supports_streaming: true,
                    supports_tools: true,
                    supports_vision: false,
                    input_cost_per_1k: None,
                    output_cost_per_1k: None,
                },
                ModelInfo {
                    id: "mistral".to_string(),
                    name: "Mistral".to_string(),
                    provider: "Ollama".to_string(),
                    context_length: 32_000,
                    supports_streaming: true,
                    supports_tools: true,
                    supports_vision: false,
                    input_cost_per_1k: None,
                    output_cost_per_1k: None,
                },
                ModelInfo {
                    id: "codellama".to_string(),
                    name: "Code Llama".to_string(),
                    provider: "Ollama".to_string(),
                    context_length: 16_000,
                    supports_streaming: true,
                    supports_tools: false,
                    supports_vision: false,
                    input_cost_per_1k: None,
                    output_cost_per_1k: None,
                },
                ModelInfo {
                    id: "qwen2.5".to_string(),
                    name: "Qwen 2.5".to_string(),
                    provider: "Ollama".to_string(),
                    context_length: 32_000,
                    supports_streaming: true,
                    supports_tools: true,
                    supports_vision: false,
                    input_cost_per_1k: None,
                    output_cost_per_1k: None,
                },
            ]
        } else {
            self.cached_models.clone()
        }
    }

    fn default_model(&self) -> &str {
        self.config.default_model.as_deref().unwrap_or("llama3.2")
    }

    fn is_configured(&self) -> bool {
        // Ollama is always "configured" - it's local
        true
    }

    async fn complete(&self, request: AIRequest) -> Result<AIResponse, AIError> {
        // Use chat API for better results
        let api_request = self.build_chat_request(&request);

        let response = self
            .client
            .post(format!("{}/api/chat", self.base_url()))
            .json(&api_request)
            .send()
            .await
            .map_err(|e| {
                if e.is_connect() {
                    AIError::Network("Cannot connect to Ollama. Is it running?".to_string())
                } else {
                    AIError::Network(e.to_string())
                }
            })?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIError::Provider(error_text));
        }

        let api_response: OllamaChatResponse = response
            .json()
            .await
            .map_err(|e| AIError::Provider(format!("Invalid response: {}", e)))?;

        Ok(AIResponse {
            id: uuid::Uuid::new_v4().to_string(),
            model: api_response.model,
            content: api_response.message.content,
            stop_reason: Some(if api_response.done {
                StopReason::EndTurn
            } else {
                StopReason::MaxTokens
            }),
            tool_calls: Vec::new(),
            usage: Some(Usage {
                input_tokens: api_response.prompt_eval_count.unwrap_or(0),
                output_tokens: api_response.eval_count.unwrap_or(0),
                total_tokens: api_response.prompt_eval_count.unwrap_or(0)
                    + api_response.eval_count.unwrap_or(0),
            }),
        })
    }

    async fn stream(&self, request: AIRequest) -> Result<AIStream, AIError> {
        let mut api_request = self.build_chat_request(&request);
        api_request.stream = true;

        let response = self
            .client
            .post(format!("{}/api/chat", self.base_url()))
            .json(&api_request)
            .send()
            .await
            .map_err(|e| {
                if e.is_connect() {
                    AIError::Network("Cannot connect to Ollama. Is it running?".to_string())
                } else {
                    AIError::Network(e.to_string())
                }
            })?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIError::Provider(error_text));
        }

        let stream = response.bytes_stream().map(move |chunk| {
            match chunk {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    // Ollama sends newline-delimited JSON
                    for line in text.lines() {
                        if let Ok(event) = serde_json::from_str::<OllamaStreamEvent>(line) {
                            if event.done {
                                return Ok(StreamChunk::Stop(StopReason::EndTurn));
                            }
                            if let Some(message) = event.message {
                                return Ok(StreamChunk::Text(message.content));
                            }
                        }
                    }
                    Ok(StreamChunk::Text(String::new()))
                }
                Err(e) => Err(AIError::Network(e.to_string())),
            }
        });

        Ok(Box::pin(stream))
    }

    fn count_tokens(&self, text: &str) -> usize {
        // Rough estimate for Llama-based tokenizers
        text.len() / 4
    }
}
