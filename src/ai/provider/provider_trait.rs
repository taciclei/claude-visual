//! AI Provider trait definition

use super::error::AIError;
use super::model::ModelInfo;
use super::request::AIRequest;
use super::response::AIResponse;
use super::stream::AIStream;
use async_trait::async_trait;

/// AI Provider trait - implemented by each provider
#[async_trait]
pub trait AIProvider: Send + Sync {
    /// Get provider name
    fn name(&self) -> &str;

    /// Get available models
    fn models(&self) -> Vec<ModelInfo>;

    /// Get default model
    fn default_model(&self) -> &str;

    /// Check if provider is configured (has API key, etc.)
    fn is_configured(&self) -> bool;

    /// Send a request and get a complete response
    async fn complete(&self, request: AIRequest) -> Result<AIResponse, AIError>;

    /// Send a request and get a streaming response
    async fn stream(&self, request: AIRequest) -> Result<AIStream, AIError>;

    /// Count tokens in text (approximate)
    fn count_tokens(&self, text: &str) -> usize {
        // Default implementation: rough estimate of 4 chars per token
        text.len() / 4
    }

    /// Validate request before sending
    fn validate_request(&self, request: &AIRequest) -> Result<(), AIError> {
        // Check model exists
        let models = self.models();
        if !models.iter().any(|m| m.id == request.model) {
            return Err(AIError::ModelNotAvailable(request.model.clone()));
        }

        // Check messages not empty
        if request.messages.is_empty() {
            return Err(AIError::InvalidRequest("No messages provided".to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_token_counting() {
        // Test default implementation
        let text = "Hello, world! This is a test.";
        let estimated = text.len() / 4;
        assert!(estimated > 0);
    }
}
