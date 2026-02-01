use crate::ai::provider::{ModelInfo, ProviderConfig};
use gpui::*;

pub(crate) struct SimpleColors {
    pub(crate) surface: Hsla,
    pub(crate) surface_hover: Hsla,
    pub(crate) border: Hsla,
    pub(crate) text: Hsla,
    pub(crate) text_muted: Hsla,
    pub(crate) accent: Hsla,
    pub(crate) error: Hsla,
    pub(crate) success: Hsla,
    pub(crate) warning: Hsla,
    pub(crate) background: Hsla,
}

/// Event emitted by model selector
#[derive(Clone, Debug)]
pub enum ModelSelectorEvent {
    /// Model selected
    ModelSelected {
        provider: String,
        model_id: String,
    },
    /// Provider configuration changed
    ProviderConfigChanged {
        provider: String,
        config: ProviderConfig,
    },
}

/// Model category for grouping
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModelCategory {
    /// Cloud providers (Claude, OpenAI)
    Cloud,
    /// Local models (Ollama)
    Local,
}

/// Provider info with models
#[derive(Clone, Debug)]
pub struct ProviderInfo {
    pub name: String,
    pub category: ModelCategory,
    pub models: Vec<ModelInfo>,
    pub is_configured: bool,
    pub config: ProviderConfig,
}
