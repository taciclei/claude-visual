//! Signature help types

use super::documentation::Documentation;
use serde::{Deserialize, Serialize};

/// Signature help
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHelp {
    /// Signatures
    pub signatures: Vec<SignatureInformation>,
    /// Active signature index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_signature: Option<u32>,
    /// Active parameter index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_parameter: Option<u32>,
}

/// Signature information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureInformation {
    /// Label
    pub label: String,
    /// Documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
    /// Parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterInformation>>,
}

/// Parameter information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInformation {
    /// Label (string or [start, end] offsets)
    pub label: ParameterLabel,
    /// Documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Documentation>,
}

/// Parameter label
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParameterLabel {
    String(String),
    Offsets([u32; 2]),
}

/// Signature help options
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHelpOptions {
    /// Trigger characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_characters: Option<Vec<String>>,
    /// Retrigger characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrigger_characters: Option<Vec<String>>,
}
