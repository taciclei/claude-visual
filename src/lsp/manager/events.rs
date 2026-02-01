//! LSP Manager events

use crate::lsp::protocol::Diagnostic;
use super::language::Language;

/// LSP Manager events
#[derive(Debug, Clone)]
pub enum LspManagerEvent {
    /// Server started
    ServerStarted(Language),
    /// Server stopped
    ServerStopped(Language),
    /// Server error
    ServerError(Language, String),
    /// Diagnostics received
    Diagnostics(String, Vec<Diagnostic>),
}
