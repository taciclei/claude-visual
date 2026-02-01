//! Diagnostics Panel
//!
//! Displays LSP diagnostics (errors, warnings, hints) for files.

mod types;
mod core;
mod render;

pub use types::{DiagnosticsPanelEvent, FileDiagnostics};
pub use core::DiagnosticsPanel;
