//! Claude provider implementation

use crate::ai::provider::ProviderConfig;

mod complete;
mod conversion;
mod models;
mod stream;
mod trait_impl;

pub(crate) use conversion::{convert_message, convert_tool};

/// Claude API provider
pub struct ClaudeProvider {
    /// HTTP client
    pub(super) client: reqwest::Client,
    /// Configuration
    pub(super) config: ProviderConfig,
}

impl ClaudeProvider {
    /// Create a new Claude provider
    pub fn new(config: ProviderConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(
                config.timeout_secs.unwrap_or(120),
            ))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    /// Create from API key
    pub fn from_api_key(api_key: impl Into<String>) -> Self {
        Self::new(ProviderConfig {
            api_key: Some(api_key.into()),
            ..Default::default()
        })
    }

    /// Get API base URL
    pub(super) fn base_url(&self) -> &str {
        self.config
            .base_url
            .as_deref()
            .unwrap_or("https://api.anthropic.com")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::provider::Message;

    #[test]
    fn test_provider_creation() {
        let provider = ClaudeProvider::from_api_key("test-key");
        assert_eq!(provider.name(), "Claude");
        assert!(provider.is_configured());
    }

    #[test]
    fn test_models_list() {
        let provider = ClaudeProvider::new(ProviderConfig::default());
        let models = provider.models();
        assert!(!models.is_empty());
        assert!(models.iter().any(|m| m.id.contains("sonnet")));
    }

    #[test]
    fn test_message_conversion() {
        let msg = Message::user("Hello");
        let converted = convert_message(&msg);
        assert_eq!(converted.role, "user");
        assert_eq!(converted.content, "Hello");
    }
}
