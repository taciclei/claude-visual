//! LSP Manager events

use super::language::Language;
use crate::lsp::protocol::Diagnostic;

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
