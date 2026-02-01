//! Tests for OpenAI provider

use super::provider::OpenAIProvider;
use crate::ai::provider::{AIProvider, ProviderConfig};

#[test]
fn test_provider_creation() {
    let provider = OpenAIProvider::from_api_key("test-key");
    assert_eq!(provider.name(), "OpenAI");
    assert!(provider.is_configured());
}

#[test]
fn test_models_list() {
    let provider = OpenAIProvider::new(ProviderConfig::default());
    let models = provider.models();
    assert!(!models.is_empty());
    assert!(models.iter().any(|m| m.id == "gpt-4o"));
}
