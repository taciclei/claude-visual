//! Non-streaming completion implementation

use crate::ai::claude::types::{ClaudeMessage, ClaudeRequest, ClaudeResponse, ContentBlock};
use crate::ai::provider::{
    AIError, AIRequest, AIResponse, MessageRole, StopReason, ToolCall, Usage,
};

use super::{convert_message, convert_tool, ClaudeProvider};

impl ClaudeProvider {
    /// Complete a request (non-streaming)
    pub(super) async fn complete_request(&self, request: AIRequest) -> Result<AIResponse, AIError> {
        let api_key = self
            .config
            .api_key
            .as_ref()
            .ok_or(AIError::Auth("No API key configured".to_string()))?;

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
            stream: false,
            tools: request
                .tools
                .as_ref()
                .map(|tools| tools.iter().map(convert_tool).collect()),
        };

        let response = self
            .client
            .post(format!("{}/v1/messages", self.base_url()))
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&claude_request)
            .send()
            .await
            .map_err(|e| AIError::Network(e.to_string()))?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AIError::Auth("Invalid API key".to_string()));
        }

        if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(AIError::RateLimit);
        }

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIError::Provider(error_text));
        }

        let claude_response: ClaudeResponse = response
            .json()
            .await
            .map_err(|e| AIError::Unknown(e.to_string()))?;

        // Convert response
        let mut content = String::new();
        let mut tool_calls = Vec::new();

        for block in claude_response.content {
            match block {
                ContentBlock::Text { text } => {
                    content.push_str(&text);
                }
                ContentBlock::ToolUse { id, name, input } => {
                    tool_calls.push(ToolCall {
                        id,
                        name,
                        arguments: input,
                    });
                }
            }
        }

        let stop_reason = match claude_response.stop_reason.as_deref() {
            Some("end_turn") => Some(StopReason::EndTurn),
            Some("stop_sequence") => Some(StopReason::StopSequence),
            Some("max_tokens") => Some(StopReason::MaxTokens),
            Some("tool_use") => Some(StopReason::ToolUse),
            _ => None,
        };

        Ok(AIResponse {
            id: claude_response.id,
            model: claude_response.model,
            content,
            stop_reason,
            tool_calls,
            usage: Some(Usage {
                input_tokens: claude_response.usage.input_tokens,
                output_tokens: claude_response.usage.output_tokens,
                total_tokens: claude_response.usage.input_tokens
                    + claude_response.usage.output_tokens,
            }),
        })
    }
}
