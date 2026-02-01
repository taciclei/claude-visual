//! Code action and command types

use super::diagnostics::Diagnostic;
use super::types::TextEdit;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Code action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeAction {
    /// Title
    pub title: String,
    /// Kind
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Diagnostics this action resolves
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Vec<Diagnostic>>,
    /// Edit to apply
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit: Option<WorkspaceEdit>,
    /// Command to execute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Command>,
}

/// Workspace edit
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkspaceEdit {
    /// Changes by document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<HashMap<String, Vec<TextEdit>>>,
}

/// Command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    /// Title
    pub title: String,
    /// Command identifier
    pub command: String,
    /// Arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<serde_json::Value>>,
}
