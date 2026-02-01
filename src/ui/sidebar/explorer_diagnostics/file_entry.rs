//! File entry with diagnostics

use crate::ui::sidebar::explorer_diagnostics::{
    BadgeStyle, DiagnosticBadge, DiagnosticCounts, ExplorerDiagnosticsStore, IconDecoration,
};
use std::path::PathBuf;

/// Explorer file entry with diagnostics
#[derive(Debug, Clone)]
pub struct FileEntryWithDiagnostics {
    /// File path
    pub path: PathBuf,
    /// File name
    pub name: String,
    /// Is directory
    pub is_dir: bool,
    /// Diagnostic counts
    pub counts: DiagnosticCounts,
    /// Badge to display
    pub badge: Option<DiagnosticBadge>,
    /// Icon decoration
    pub decoration: IconDecoration,
}

impl FileEntryWithDiagnostics {
    /// Create from path and store
    pub fn new(path: PathBuf, store: &mut ExplorerDiagnosticsStore) -> Self {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let is_dir = path.is_dir();
        let counts = store.get_counts(&path);

        let config = store.config();
        let style = if config.show_counts {
            BadgeStyle::Count
        } else {
            BadgeStyle::Dot
        };

        let badge = if config.show_error_badges || config.show_warning_badges {
            DiagnosticBadge::from_counts(&counts, style)
        } else {
            None
        };

        let decoration = if config.show_icon_decorations {
            IconDecoration::from_counts(&counts)
        } else {
            IconDecoration::None
        };

        Self {
            path,
            name,
            is_dir,
            counts,
            badge,
            decoration,
        }
    }
}
