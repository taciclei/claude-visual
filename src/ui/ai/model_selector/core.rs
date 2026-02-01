use super::types::{ModelCategory, ModelSelectorEvent, ProviderInfo};
use crate::ai::provider::{ModelInfo, ProviderConfig};
use gpui::*;

/// Model selector state
pub struct ModelSelector {
    /// Available providers with their models
    pub(crate) providers: Vec<ProviderInfo>,
    /// Currently selected provider index
    pub(crate) selected_provider_idx: usize,
    /// Currently selected model ID
    pub(crate) selected_model_id: String,
    /// Whether the dropdown is expanded
    pub(crate) is_expanded: bool,
    /// Search filter
    pub(crate) search_query: String,
    /// Focus handle
    pub(crate) focus_handle: FocusHandle,
    /// Show only configured providers
    pub(crate) show_configured_only: bool,
}

impl ModelSelector {
    pub fn new(cx: &mut Context<Self>) -> Self {
        // Initialize with default providers
        let providers = vec![
            ProviderInfo {
                name: "Claude".to_string(),
                category: ModelCategory::Cloud,
                models: vec![
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
                ],
                is_configured: false,
                config: ProviderConfig::default(),
            },
            ProviderInfo {
                name: "OpenAI".to_string(),
                category: ModelCategory::Cloud,
                models: vec![
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
                ],
                is_configured: false,
                config: ProviderConfig::default(),
            },
            ProviderInfo {
                name: "Ollama".to_string(),
                category: ModelCategory::Local,
                models: vec![
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
                ],
                is_configured: true, // Ollama doesn't need API key
                config: ProviderConfig::default(),
            },
        ];

        Self {
            providers,
            selected_provider_idx: 0,
            selected_model_id: "claude-3-5-sonnet-20241022".to_string(),
            is_expanded: false,
            search_query: String::new(),
            focus_handle: cx.focus_handle(),
            show_configured_only: false,
        }
    }

    /// Toggle dropdown expansion
    pub fn toggle_expanded(&mut self, cx: &mut Context<Self>) {
        self.is_expanded = !self.is_expanded;
        cx.notify();
    }

    /// Select a model
    pub fn select_model(&mut self, provider_idx: usize, model_id: String, cx: &mut Context<Self>) {
        self.selected_provider_idx = provider_idx;
        self.selected_model_id = model_id.clone();
        self.is_expanded = false;

        let provider = &self.providers[provider_idx];
        cx.emit(ModelSelectorEvent::ModelSelected {
            provider: provider.name.clone(),
            model_id,
        });
        cx.notify();
    }

    /// Update provider configuration
    pub fn update_provider_config(
        &mut self,
        provider_name: &str,
        config: ProviderConfig,
        cx: &mut Context<Self>,
    ) {
        if let Some(provider) = self.providers.iter_mut().find(|p| p.name == provider_name) {
            provider.config = config.clone();
            provider.is_configured = config.api_key.is_some();
            cx.emit(ModelSelectorEvent::ProviderConfigChanged {
                provider: provider_name.to_string(),
                config,
            });
            cx.notify();
        }
    }

    /// Get currently selected model info
    pub fn selected_model(&self) -> Option<&ModelInfo> {
        let provider = self.providers.get(self.selected_provider_idx)?;
        provider
            .models
            .iter()
            .find(|m| m.id == self.selected_model_id)
    }

    /// Get selected provider name
    pub fn selected_provider_name(&self) -> &str {
        self.providers
            .get(self.selected_provider_idx)
            .map(|p| p.name.as_str())
            .unwrap_or("Unknown")
    }

    /// Filter models by search query
    pub(crate) fn filtered_models(&self) -> Vec<(usize, &ModelInfo)> {
        let query = self.search_query.to_lowercase();
        let mut results = Vec::new();

        for (idx, provider) in self.providers.iter().enumerate() {
            if self.show_configured_only && !provider.is_configured {
                continue;
            }

            for model in &provider.models {
                if query.is_empty()
                    || model.name.to_lowercase().contains(&query)
                    || model.id.to_lowercase().contains(&query)
                    || provider.name.to_lowercase().contains(&query)
                {
                    results.push((idx, model));
                }
            }
        }

        results
    }
}
