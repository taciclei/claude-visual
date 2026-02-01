//! Core DAP data types

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Source reference
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    /// Source name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Source path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Source reference (for adapter-managed sources)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_reference: Option<i64>,
    /// Presentation hint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation_hint: Option<String>,
    /// Origin of the source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}

/// Breakpoint location
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakpointLocation {
    /// Line number
    pub line: i64,
    /// Optional column
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
    /// Optional end line
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i64>,
    /// Optional end column
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_column: Option<i64>,
}

/// Breakpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Breakpoint {
    /// Breakpoint ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Is breakpoint verified
    pub verified: bool,
    /// Error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// Line
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    /// Column
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
    /// End line
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i64>,
    /// End column
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_column: Option<i64>,
    /// Instruction reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction_reference: Option<String>,
    /// Offset from instruction reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

/// Stack frame
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StackFrame {
    /// Frame ID
    pub id: i64,
    /// Frame name
    pub name: String,
    /// Source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// Line number
    pub line: i64,
    /// Column
    pub column: i64,
    /// End line
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i64>,
    /// End column
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_column: Option<i64>,
    /// Module ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_id: Option<JsonValue>,
    /// Presentation hint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation_hint: Option<String>,
}

/// Scope
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scope {
    /// Scope name
    pub name: String,
    /// Presentation hint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation_hint: Option<String>,
    /// Variables reference
    pub variables_reference: i64,
    /// Named variables
    #[serde(default)]
    pub named_variables: i64,
    /// Indexed variables
    #[serde(default)]
    pub indexed_variables: i64,
    /// Is expensive
    #[serde(default)]
    pub expensive: bool,
    /// Source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// Line
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    /// Column
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
    /// End line
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i64>,
    /// End column
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_column: Option<i64>,
}

/// Variable
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variable {
    /// Variable name
    pub name: String,
    /// Variable value
    pub value: String,
    /// Variable type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub var_type: Option<String>,
    /// Presentation hint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation_hint: Option<VariablePresentationHint>,
    /// Evaluate name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluate_name: Option<String>,
    /// Variables reference (for nested variables)
    pub variables_reference: i64,
    /// Named variables
    #[serde(default)]
    pub named_variables: i64,
    /// Indexed variables
    #[serde(default)]
    pub indexed_variables: i64,
    /// Memory reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reference: Option<String>,
}

/// Variable presentation hint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariablePresentationHint {
    /// Kind (property, method, class, data, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Attributes (static, constant, readOnly, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
    /// Visibility (public, private, protected, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

/// Thread
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thread {
    /// Thread ID
    pub id: i64,
    /// Thread name
    pub name: String,
}
