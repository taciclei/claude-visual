//! OpenAI model definitions

use crate::ai::provider::ModelInfo;

/// Get available OpenAI models
pub(crate) fn get_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "gpt-4o".to_string(),
            name: "GPT-4o".to_string(),
            provider: "OpenAI".to_string(),
            context_length: 128_000,
            supports_streaming: true,
            supports_tools: true,
            supports_vision: true,
            input_cost_per_1k: Some(0.005),
            output_cost_per_1k: Some(0.015),
        },
        ModelInfo {
            id: "gpt-4o-mini".to_string(),
            name: "GPT-4o Mini".to_string(),
            provider: "OpenAI".to_string(),
            context_length: 128_000,
            supports_streaming: true,
            supports_tools: true,
            supports_vision: true,
            input_cost_per_1k: Some(0.00015),
            output_cost_per_1k: Some(0.0006),
        },
        ModelInfo {
            id: "gpt-4-turbo".to_string(),
            name: "GPT-4 Turbo".to_string(),
            provider: "OpenAI".to_string(),
            context_length: 128_000,
            supports_streaming: true,
            supports_tools: true,
            supports_vision: true,
            input_cost_per_1k: Some(0.01),
            output_cost_per_1k: Some(0.03),
        },
        ModelInfo {
            id: "gpt-3.5-turbo".to_string(),
            name: "GPT-3.5 Turbo".to_string(),
            provider: "OpenAI".to_string(),
            context_length: 16_385,
            supports_streaming: true,
            supports_tools: true,
            supports_vision: false,
            input_cost_per_1k: Some(0.0005),
            output_cost_per_1k: Some(0.0015),
        },
        ModelInfo {
            id: "o1".to_string(),
            name: "o1".to_string(),
            provider: "OpenAI".to_string(),
            context_length: 200_000,
            supports_streaming: true,
            supports_tools: true,
            supports_vision: true,
            input_cost_per_1k: Some(0.015),
            output_cost_per_1k: Some(0.06),
        },
        ModelInfo {
            id: "o1-mini".to_string(),
            name: "o1 Mini".to_string(),
            provider: "OpenAI".to_string(),
            context_length: 128_000,
            supports_streaming: true,
            supports_tools: true,
            supports_vision: false,
            input_cost_per_1k: Some(0.003),
            output_cost_per_1k: Some(0.012),
        },
    ]
}
