//! Diagnostics Panel
//!
//! Displays LSP diagnostics (errors, warnings, hints) for files.

mod core;
mod render;
mod types;

pub use core::DiagnosticsPanel;
pub use types::{DiagnosticsPanelEvent, FileDiagnostics};
