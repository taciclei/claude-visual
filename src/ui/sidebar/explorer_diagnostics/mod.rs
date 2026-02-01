//! File Explorer Diagnostics Integration
//!
//! Provides inline diagnostics display in the file explorer,
//! showing error and warning counts per file.

mod badge;
mod file_entry;
mod store;
mod types;

#[cfg(test)]
mod tests;

pub use badge::DiagnosticBadge;
pub use file_entry::FileEntryWithDiagnostics;
pub use store::ExplorerDiagnosticsStore;
pub use types::{BadgeStyle, DiagnosticCounts, ExplorerDiagnosticsConfig, IconDecoration};
