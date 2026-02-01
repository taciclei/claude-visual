//! Ollama Provider
//!
//! Integration with local Ollama models.

mod core;
mod provider;
mod types;

pub use core::OllamaProvider;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        let provider = OllamaProvider::local();
        use crate::ai::provider::AIProvider;
        assert_eq!(provider.name(), "Ollama");
        assert!(provider.is_configured());
    }

    #[test]
    fn test_context_estimation() {
        let provider = OllamaProvider::local();
        assert_eq!(provider.estimate_context_length("llama3-128k"), 128_000);
        assert_eq!(provider.estimate_context_length("mistral"), 32_000);
        assert_eq!(provider.estimate_context_length("phi3"), 2_048);
    }

    #[test]
    fn test_models_list() {
        let provider = OllamaProvider::local();
        use crate::ai::provider::AIProvider;
        let models = provider.models();
        assert!(!models.is_empty());
    }
}
