//! LSP Module
//!
//! Language Server Protocol client for code intelligence features.

pub mod client;
pub mod manager;
pub mod protocol;

pub use client::{LspClient, LspClientConfig};
pub use manager::{LanguageServer, LspManager};
pub use protocol::{
    CompletionItem, Diagnostic, DiagnosticSeverity, DocumentSymbol, Hover, Location, Position,
    Range, SignatureHelp, TextDocumentIdentifier,
};
