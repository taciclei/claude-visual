//! Claude model definitions

use crate::ai::provider::ModelInfo;

use super::ClaudeProvider;

impl ClaudeProvider {
    /// Get available models
    pub(super) fn get_models() -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "claude-3-opus-20240229".to_string(),
                name: "Claude 3 Opus".to_string(),
                provider: "Anthropic".to_string(),
                context_length: 200_000,
                supports_streaming: true,
                supports_tools: true,
                supports_vision: true,
                input_cost_per_1k: Some(0.015),
                output_cost_per_1k: Some(0.075),
            },
            ModelInfo {
                id: "claude-3-sonnet-20240229".to_string(),
                name: "Claude 3 Sonnet".to_string(),
                provider: "Anthropic".to_string(),
                context_length: 200_000,
                supports_streaming: true,
                supports_tools: true,
                supports_vision: true,
                input_cost_per_1k: Some(0.003),
                output_cost_per_1k: Some(0.015),
            },
            ModelInfo {
                id: "claude-3-haiku-20240307".to_string(),
                name: "Claude 3 Haiku".to_string(),
                provider: "Anthropic".to_string(),
                context_length: 200_000,
                supports_streaming: true,
                supports_tools: true,
                supports_vision: true,
                input_cost_per_1k: Some(0.00025),
                output_cost_per_1k: Some(0.00125),
            },
            ModelInfo {
                id: "claude-3-5-sonnet-20241022".to_string(),
                name: "Claude 3.5 Sonnet".to_string(),
                provider: "Anthropic".to_string(),
                context_length: 200_000,
                supports_streaming: true,
                supports_tools: true,
                supports_vision: true,
                input_cost_per_1k: Some(0.003),
                output_cost_per_1k: Some(0.015),
            },
        ]
    }
}
