//! Diagnostic types for errors, warnings, and related information

use super::types::{Location, Range};
use serde::{Deserialize, Serialize};

/// Diagnostic severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

/// A diagnostic (error, warning, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    /// Range of the diagnostic
    pub range: Range,
    /// Severity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<DiagnosticSeverity>,
    /// Diagnostic code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<DiagnosticCode>,
    /// Source of the diagnostic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Human-readable message
    pub message: String,
    /// Related information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_information: Option<Vec<DiagnosticRelatedInformation>>,
}

/// Diagnostic code (can be string or number)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DiagnosticCode {
    Number(i32),
    String(String),
}

/// Related diagnostic information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticRelatedInformation {
    /// Location
    pub location: Location,
    /// Message
    pub message: String,
}

/// Publish diagnostics params
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishDiagnosticsParams {
    /// Document URI
    pub uri: String,
    /// Diagnostics
    pub diagnostics: Vec<Diagnostic>,
    /// Document version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}
