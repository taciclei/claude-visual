//! Provider configuration

use serde::{Deserialize, Serialize};

/// Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// API key
    pub api_key: Option<String>,
    /// API base URL (for custom endpoints)
    pub base_url: Option<String>,
    /// Organization ID (for OpenAI)
    pub organization_id: Option<String>,
    /// Default model
    pub default_model: Option<String>,
    /// Timeout in seconds
    pub timeout_secs: Option<u64>,
    /// Maximum retries
    pub max_retries: Option<u32>,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            base_url: None,
            organization_id: None,
            default_model: None,
            timeout_secs: Some(120),
            max_retries: Some(3),
        }
    }
}
