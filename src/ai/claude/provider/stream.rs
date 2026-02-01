//! Streaming completion implementation

use futures::StreamExt;

use crate::ai::claude::types::{ClaudeMessage, ClaudeRequest, StreamEvent};
use crate::ai::provider::{AIError, AIRequest, AIStream, MessageRole, StopReason, StreamChunk};

use super::{convert_message, convert_tool, ClaudeProvider};

impl ClaudeProvider {
    /// Stream a request
    pub(super) async fn stream_request(&self, request: AIRequest) -> Result<AIStream, AIError> {
        let api_key = self
            .config
            .api_key
            .as_ref()
            .ok_or(AIError::Auth("No API key configured".to_string()))?
            .clone();

        // Build Claude request
        let mut messages: Vec<ClaudeMessage> = Vec::new();

        for msg in &request.messages {
            if msg.role != MessageRole::System {
                messages.push(convert_message(msg));
            }
        }

        let claude_request = ClaudeRequest {
            model: request.model.clone(),
            messages,
            system: request.system.clone(),
            max_tokens: request.max_tokens.unwrap_or(4096),
            temperature: request.temperature,
            top_p: request.top_p,
            stop_sequences: request.stop.clone(),
            stream: true,
            tools: request
                .tools
                .as_ref()
                .map(|tools| tools.iter().map(convert_tool).collect()),
        };

        let base_url = self.base_url().to_string();

        let response = self
            .client
            .post(format!("{}/v1/messages", base_url))
            .header("x-api-key", &api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&claude_request)
            .send()
            .await
            .map_err(|e| AIError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIError::Provider(error_text));
        }

        let stream = response.bytes_stream().map(move |result| {
            match result {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    // Parse SSE events
                    for line in text.lines() {
                        if line.starts_with("data: ") {
                            let json = &line[6..];
                            if json == "[DONE]" {
                                return Ok(StreamChunk::Stop(StopReason::EndTurn));
                            }
                            if let Ok(event) = serde_json::from_str::<StreamEvent>(json) {
                                match event {
                                    StreamEvent::ContentBlockDelta { delta, .. } => {
                                        if let Some(text) = delta.text {
                                            return Ok(StreamChunk::Text(text));
                                        }
                                    }
                                    StreamEvent::MessageStop => {
                                        return Ok(StreamChunk::Stop(StopReason::EndTurn));
                                    }
                                    _ => {}
                                }
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
}
