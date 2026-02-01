//! AIProvider trait implementation for ClaudeProvider

use async_trait::async_trait;

use crate::ai::provider::{AIError, AIProvider, AIRequest, AIResponse, AIStream, ModelInfo};

use super::ClaudeProvider;

#[async_trait]
impl AIProvider for ClaudeProvider {
    fn name(&self) -> &str {
        "Claude"
    }

    fn models(&self) -> Vec<ModelInfo> {
        Self::get_models()
    }

    fn default_model(&self) -> &str {
        self.config
            .default_model
            .as_deref()
            .unwrap_or("claude-3-5-sonnet-20241022")
    }

    fn is_configured(&self) -> bool {
        self.config.api_key.is_some()
    }

    async fn complete(&self, request: AIRequest) -> Result<AIResponse, AIError> {
        self.validate_request(&request)?;
        self.complete_request(request).await
    }

    async fn stream(&self, request: AIRequest) -> Result<AIStream, AIError> {
        self.validate_request(&request)?;
        self.stream_request(request).await
    }
}
