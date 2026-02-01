//! Initialization protocol types

use serde::{Deserialize, Serialize};
use super::capabilities::{ClientCapabilities, ServerCapabilities};

/// Initialize params
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    /// Process ID
    pub process_id: Option<u32>,
    /// Root URI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_uri: Option<String>,
    /// Client capabilities
    pub capabilities: ClientCapabilities,
}

/// Initialize result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeResult {
    /// Server capabilities
    pub capabilities: ServerCapabilities,
}
