//! DAP request argument types

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

fn default_true() -> bool {
    true
}

/// Initialize request arguments
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeArguments {
    /// The ID of the client using this adapter
    pub client_id: Option<String>,
    /// The human-readable name of the client
    pub client_name: Option<String>,
    /// The ID of the debug adapter
    pub adapter_id: String,
    /// The ISO-639 locale of the client
    pub locale: Option<String>,
    /// If true, lines start at 1
    #[serde(default = "default_true")]
    pub lines_start_at1: bool,
    /// If true, columns start at 1
    #[serde(default = "default_true")]
    pub columns_start_at1: bool,
    /// Determines in what format paths are specified
    pub path_format: Option<String>,
    /// Client supports variable type
    #[serde(default)]
    pub supports_variable_type: bool,
    /// Client supports paging variables
    #[serde(default)]
    pub supports_variable_paging: bool,
    /// Client supports runInTerminal request
    #[serde(default)]
    pub supports_run_in_terminal_request: bool,
    /// Client supports memory references
    #[serde(default)]
    pub supports_memory_references: bool,
    /// Client supports progress reporting
    #[serde(default)]
    pub supports_progress_reporting: bool,
    /// Client supports invalidated event
    #[serde(default)]
    pub supports_invalidated_event: bool,
}

impl Default for InitializeArguments {
    fn default() -> Self {
        Self {
            client_id: Some("claude-visual".to_string()),
            client_name: Some("Claude Visual".to_string()),
            adapter_id: "".to_string(),
            locale: Some("en".to_string()),
            lines_start_at1: true,
            columns_start_at1: true,
            path_format: Some("path".to_string()),
            supports_variable_type: true,
            supports_variable_paging: false,
            supports_run_in_terminal_request: true,
            supports_memory_references: false,
            supports_progress_reporting: true,
            supports_invalidated_event: true,
        }
    }
}

/// Launch request arguments
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchArguments {
    /// If true, the launch request should not launch but just prepare for launch
    #[serde(default)]
    pub no_debug: bool,
    /// Program to launch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<String>,
    /// Program arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// Working directory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    /// Environment variables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    /// Stop at entry point
    #[serde(default)]
    pub stop_on_entry: bool,
    /// Additional launch configuration (adapter-specific)
    #[serde(flatten)]
    pub additional: HashMap<String, JsonValue>,
}

impl Default for LaunchArguments {
    fn default() -> Self {
        Self {
            no_debug: false,
            program: None,
            args: None,
            cwd: None,
            env: None,
            stop_on_entry: false,
            additional: HashMap::new(),
        }
    }
}

/// Attach request arguments
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachArguments {
    /// Process ID to attach to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<i64>,
    /// Port to connect to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    /// Host to connect to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Additional configuration
    #[serde(flatten)]
    pub additional: HashMap<String, JsonValue>,
}
