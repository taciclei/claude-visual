//! LSP Module
//!
//! Language Server Protocol client for code intelligence features.

pub mod client;
pub mod protocol;
pub mod manager;

pub use client::{LspClient, LspClientConfig};
pub use protocol::{
    Position, Range, Location, TextDocumentIdentifier,
    CompletionItem, Diagnostic, DiagnosticSeverity,
    Hover, SignatureHelp, DocumentSymbol,
};
pub use manager::{LspManager, LanguageServer};
