//! Server and client capability types

use super::completion::{CompletionClientCapabilities, CompletionOptions};
use super::hover::HoverClientCapabilities;
use super::signature::SignatureHelpOptions;
use serde::{Deserialize, Serialize};

/// Server capabilities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {
    /// Text document sync options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_document_sync: Option<TextDocumentSyncKind>,
    /// Completion provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_provider: Option<CompletionOptions>,
    /// Hover provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_provider: Option<bool>,
    /// Signature help provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_help_provider: Option<SignatureHelpOptions>,
    /// Definition provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition_provider: Option<bool>,
    /// References provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references_provider: Option<bool>,
    /// Document symbol provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_symbol_provider: Option<bool>,
    /// Code action provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_action_provider: Option<bool>,
    /// Rename provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename_provider: Option<bool>,
}

/// Text document sync kind
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
pub enum TextDocumentSyncKind {
    None = 0,
    Full = 1,
    Incremental = 2,
}

/// Client capabilities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientCapabilities {
    /// Text document capabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_document: Option<TextDocumentClientCapabilities>,
}

/// Text document client capabilities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentClientCapabilities {
    /// Completion capabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion: Option<CompletionClientCapabilities>,
    /// Hover capabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover: Option<HoverClientCapabilities>,
}
