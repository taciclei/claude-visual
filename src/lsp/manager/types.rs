//! LSP Manager types

use super::language::Language;
use crate::lsp::protocol::ServerCapabilities;

/// Language server info
#[derive(Debug, Clone)]
pub struct LanguageServer {
    /// Language
    pub language: Language,
    /// Server command
    pub command: String,
    /// Server capabilities
    pub capabilities: Option<ServerCapabilities>,
    /// Whether server is running
    pub is_running: bool,
    /// Last error
    pub last_error: Option<String>,
}

/// Open document info
#[derive(Debug, Clone)]
pub(crate) struct OpenDocument {
    pub(crate) uri: String,
    pub(crate) language: Language,
    pub(crate) version: i32,
}
