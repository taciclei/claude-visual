//! Model information types

/// Model information for model switching
#[derive(Debug, Clone)]
pub struct ModelInfo {
    /// Model ID (e.g., "claude-sonnet-4-20250514")
    pub id: String,
    /// Display name (e.g., "Claude Sonnet 4")
    pub name: String,
    /// Short description
    pub description: &'static str,
    /// Icon/emoji
    pub icon: &'static str,
    /// Whether this is the current model
    pub is_current: bool,
}

impl ModelInfo {
    /// Create a list of available Claude models
    pub fn available_models() -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "claude-opus-4-5-20251101".to_string(),
                name: "Claude Opus 4.5".to_string(),
                description: "Latest frontier model - highest capability",
                icon: "🌟",
                is_current: false,
            },
            ModelInfo {
                id: "claude-sonnet-4-20250514".to_string(),
                name: "Claude Sonnet 4".to_string(),
                description: "Best balance of speed and intelligence",
                icon: "⚡",
                is_current: false,
            },
            ModelInfo {
                id: "claude-opus-4-20250514".to_string(),
                name: "Claude Opus 4".to_string(),
                description: "Highly capable for complex reasoning",
                icon: "🧠",
                is_current: false,
            },
            ModelInfo {
                id: "claude-haiku-3-5-20241022".to_string(),
                name: "Claude Haiku 3.5".to_string(),
                description: "Fastest response times, cost-effective",
                icon: "🚀",
                is_current: false,
            },
        ]
    }

    /// Get model by short name (e.g., "sonnet", "opus", "haiku")
    pub fn from_short_name(name: &str) -> Option<ModelInfo> {
        let name_lower = name.to_lowercase();
        Self::available_models().into_iter().find(|m| {
            m.name.to_lowercase().contains(&name_lower) ||
            m.id.to_lowercase().contains(&name_lower)
        })
    }
}
