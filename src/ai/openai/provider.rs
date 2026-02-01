//! OpenAI provider core implementation

use super::types::{
    OpenAIApiRequest, OpenAIFunction, OpenAIMessage, OpenAITool,
};
use super::OPENAI_API_URL;
use crate::ai::provider::{AIError, AIRequest, ProviderConfig};

/// OpenAI provider
pub struct OpenAIProvider {
    /// HTTP client
    pub(crate) client: reqwest::Client,
    /// Configuration
    pub(crate) config: ProviderConfig,
}

impl OpenAIProvider {
    /// Create a new OpenAI provider
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
    pub(crate) fn base_url(&self) -> &str {
        self.config.base_url.as_deref().unwrap_or(OPENAI_API_URL)
    }

    /// Get API key
    pub(crate) fn api_key(&self) -> Result<&str, AIError> {
        self.config
            .api_key
            .as_deref()
            .ok_or_else(|| AIError::Auth("OpenAI API key not configured".to_string()))
    }

    /// Build request headers
    pub(crate) fn build_headers(&self) -> Result<reqwest::header::HeaderMap, AIError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", self.api_key()?)
                .parse()
                .map_err(|_| AIError::Auth("Invalid API key".to_string()))?,
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );

        if let Some(org_id) = &self.config.organization_id {
            headers.insert(
                "OpenAI-Organization",
                org_id
                    .parse()
                    .map_err(|_| AIError::Auth("Invalid organization ID".to_string()))?,
            );
        }

        Ok(headers)
    }

    /// Convert internal request to OpenAI API format
    pub(crate) fn build_api_request(&self, request: &AIRequest) -> OpenAIApiRequest {
        let mut messages: Vec<OpenAIMessage> = Vec::new();

        // Add system message if present
        if let Some(system) = &request.system {
            messages.push(OpenAIMessage {
                role: "system".to_string(),
                content: Some(system.clone()),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            });
        }

        // Add conversation messages
        for msg in &request.messages {
            messages.push(OpenAIMessage {
                role: match msg.role {
                    crate::ai::provider::MessageRole::System => "system".to_string(),
                    crate::ai::provider::MessageRole::User => "user".to_string(),
                    crate::ai::provider::MessageRole::Assistant => "assistant".to_string(),
                    crate::ai::provider::MessageRole::Tool => "tool".to_string(),
                },
                content: Some(msg.content.clone()),
                name: msg.name.clone(),
                tool_calls: None,
                tool_call_id: msg.tool_call_id.clone(),
            });
        }

        OpenAIApiRequest {
            model: request.model.clone(),
            messages,
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
            stop: request.stop.clone(),
            stream: Some(request.stream),
            tools: request.tools.as_ref().map(|tools| {
                tools
                    .iter()
                    .map(|t| OpenAITool {
                        tool_type: "function".to_string(),
                        function: OpenAIFunction {
                            name: t.name.clone(),
                            description: Some(t.description.clone()),
                            parameters: t.parameters.clone(),
                        },
                    })
                    .collect()
            }),
        }
    }
}
