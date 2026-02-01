//! DAP event body types

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use super::types::Source;

/// Stopped event body
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoppedEventBody {
    /// Reason for stop
    pub reason: String,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Thread ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<i64>,
    /// Preserve focus hint
    #[serde(default)]
    pub preserve_focus_hint: bool,
    /// Additional text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// All threads stopped
    #[serde(default)]
    pub all_threads_stopped: bool,
    /// Hit breakpoint IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_breakpoint_ids: Option<Vec<i64>>,
}

/// Output event body
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputEventBody {
    /// Output category (console, stdout, stderr, telemetry)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Output text
    pub output: String,
    /// Output group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Variables reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables_reference: Option<i64>,
    /// Source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// Line
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    /// Column
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
}

/// Terminated event body
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminatedEventBody {
    /// Restart data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<JsonValue>,
}
