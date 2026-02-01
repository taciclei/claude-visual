//! File Explorer Diagnostics Integration
//!
//! Provides inline diagnostics display in the file explorer,
//! showing error and warning counts per file.

mod types;
mod badge;
mod store;
mod file_entry;

#[cfg(test)]
mod tests;

pub use types::{DiagnosticCounts, ExplorerDiagnosticsConfig, BadgeStyle, IconDecoration};
pub use badge::DiagnosticBadge;
pub use store::ExplorerDiagnosticsStore;
pub use file_entry::FileEntryWithDiagnostics;
