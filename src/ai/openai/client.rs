//! AIProvider trait implementation for OpenAI

use async_trait::async_trait;
use futures::StreamExt;

use super::models::get_models;
use super::provider::OpenAIProvider;
use super::types::{OpenAIApiResponse, OpenAIStreamEvent};
use crate::ai::provider::{
    AIError, AIProvider, AIRequest, AIResponse, AIStream, ModelInfo, StopReason, StreamChunk,
    ToolCall, Usage,
};

#[async_trait]
impl AIProvider for OpenAIProvider {
    fn name(&self) -> &str {
        "OpenAI"
    }

    fn models(&self) -> Vec<ModelInfo> {
        get_models()
    }

    fn default_model(&self) -> &str {
        self.config.default_model.as_deref().unwrap_or("gpt-4o")
    }

    fn is_configured(&self) -> bool {
        self.config.api_key.is_some()
    }

    async fn complete(&self, request: AIRequest) -> Result<AIResponse, AIError> {
        self.validate_request(&request)?;

        let mut api_request = self.build_api_request(&request);
        api_request.stream = Some(false);

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url()))
            .headers(self.build_headers()?)
            .json(&api_request)
            .send()
            .await
            .map_err(|e| AIError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();

            return match status.as_u16() {
                401 => Err(AIError::Auth("Invalid API key".to_string())),
                429 => Err(AIError::RateLimit),
                _ => Err(AIError::Provider(format!("{}: {}", status, error_text))),
            };
        }

        let api_response: OpenAIApiResponse = response
            .json()
            .await
            .map_err(|e| AIError::Provider(format!("Invalid response: {}", e)))?;

        let choice = api_response
            .choices
            .first()
            .ok_or_else(|| AIError::Provider("No choices in response".to_string()))?;

        Ok(AIResponse {
            id: api_response.id,
            model: api_response.model,
            content: choice.message.content.clone().unwrap_or_default(),
            stop_reason: choice.finish_reason.as_ref().map(|r| match r.as_str() {
                "stop" => StopReason::EndTurn,
                "length" => StopReason::MaxTokens,
                "tool_calls" => StopReason::ToolUse,
                _ => StopReason::EndTurn,
            }),
            tool_calls: choice
                .message
                .tool_calls
                .as_ref()
                .map(|calls| {
                    calls
                        .iter()
                        .map(|tc| ToolCall {
                            id: tc.id.clone(),
                            name: tc.function.name.clone(),
                            arguments: serde_json::from_str(&tc.function.arguments)
                                .unwrap_or(serde_json::Value::Null),
                        })
                        .collect()
                })
                .unwrap_or_default(),
            usage: api_response.usage.map(|u| Usage {
                input_tokens: u.prompt_tokens,
                output_tokens: u.completion_tokens,
                total_tokens: u.total_tokens,
            }),
        })
    }

    async fn stream(&self, request: AIRequest) -> Result<AIStream, AIError> {
        self.validate_request(&request)?;

        let mut api_request = self.build_api_request(&request);
        api_request.stream = Some(true);

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url()))
            .headers(self.build_headers()?)
            .json(&api_request)
            .send()
            .await
            .map_err(|e| AIError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();

            return match status.as_u16() {
                401 => Err(AIError::Auth("Invalid API key".to_string())),
                429 => Err(AIError::RateLimit),
                _ => Err(AIError::Provider(format!("{}: {}", status, error_text))),
            };
        }

        let stream = response.bytes_stream().map(move |chunk| match chunk {
            Ok(bytes) => {
                let text = String::from_utf8_lossy(&bytes);
                for line in text.lines() {
                    if let Some(data) = line.strip_prefix("data: ") {
                        if data == "[DONE]" {
                            return Ok(StreamChunk::Stop(StopReason::EndTurn));
                        }
                        if let Ok(event) = serde_json::from_str::<OpenAIStreamEvent>(data) {
                            if let Some(choice) = event.choices.first() {
                                if let Some(content) = &choice.delta.content {
                                    return Ok(StreamChunk::Text(content.clone()));
                                }
                                if let Some(reason) = &choice.finish_reason {
                                    return Ok(StreamChunk::Stop(match reason.as_str() {
                                        "stop" => StopReason::EndTurn,
                                        "length" => StopReason::MaxTokens,
                                        "tool_calls" => StopReason::ToolUse,
                                        _ => StopReason::EndTurn,
                                    }));
                                }
                            }
                        }
                    }
                }
                Ok(StreamChunk::Text(String::new()))
            }
            Err(e) => Err(AIError::Network(e.to_string())),
        });

        Ok(Box::pin(stream))
    }

    fn count_tokens(&self, text: &str) -> usize {
        // OpenAI uses cl100k_base tokenizer
        // Rough estimate: ~4 characters per token
        text.len() / 4
    }
}
