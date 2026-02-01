//! Core types for explorer diagnostics

use crate::lsp::protocol::{Diagnostic, DiagnosticSeverity};
use serde::{Deserialize, Serialize};

/// Diagnostic counts for a file or directory
#[derive(Debug, Clone, Default)]
pub struct DiagnosticCounts {
    /// Number of errors
    pub errors: usize,
    /// Number of warnings
    pub warnings: usize,
    /// Number of info messages
    pub info: usize,
    /// Number of hints
    pub hints: usize,
}

impl DiagnosticCounts {
    /// Create from list of diagnostics
    pub fn from_diagnostics(diagnostics: &[Diagnostic]) -> Self {
        let mut counts = Self::default();

        for diag in diagnostics {
            match diag.severity {
                Some(DiagnosticSeverity::Error) => counts.errors += 1,
                Some(DiagnosticSeverity::Warning) => counts.warnings += 1,
                Some(DiagnosticSeverity::Information) => counts.info += 1,
                Some(DiagnosticSeverity::Hint) => counts.hints += 1,
                None => counts.info += 1,
            }
        }

        counts
    }

    /// Merge another counts into this one
    pub fn merge(&mut self, other: &DiagnosticCounts) {
        self.errors += other.errors;
        self.warnings += other.warnings;
        self.info += other.info;
        self.hints += other.hints;
    }

    /// Check if there are any problems
    pub fn has_problems(&self) -> bool {
        self.errors > 0 || self.warnings > 0
    }

    /// Check if there are any diagnostics
    pub fn has_any(&self) -> bool {
        self.errors > 0 || self.warnings > 0 || self.info > 0 || self.hints > 0
    }

    /// Get total count
    pub fn total(&self) -> usize {
        self.errors + self.warnings + self.info + self.hints
    }

    /// Get most severe level
    pub fn most_severe(&self) -> Option<DiagnosticSeverity> {
        if self.errors > 0 {
            Some(DiagnosticSeverity::Error)
        } else if self.warnings > 0 {
            Some(DiagnosticSeverity::Warning)
        } else if self.info > 0 {
            Some(DiagnosticSeverity::Information)
        } else if self.hints > 0 {
            Some(DiagnosticSeverity::Hint)
        } else {
            None
        }
    }
}

/// Configuration for explorer diagnostics display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplorerDiagnosticsConfig {
    /// Show diagnostics in explorer
    pub enabled: bool,
    /// Show error count badges
    pub show_error_badges: bool,
    /// Show warning count badges
    pub show_warning_badges: bool,
    /// Show diagnostic decoration on file icons
    pub show_icon_decorations: bool,
    /// Aggregate diagnostics to parent directories
    pub aggregate_to_parents: bool,
    /// Maximum depth for aggregation (0 = root only, -1 = unlimited)
    pub aggregation_depth: i32,
    /// Show counts in badge or just indicator dot
    pub show_counts: bool,
}

impl Default for ExplorerDiagnosticsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            show_error_badges: true,
            show_warning_badges: true,
            show_icon_decorations: true,
            aggregate_to_parents: true,
            aggregation_depth: -1,
            show_counts: true,
        }
    }
}

/// Badge display style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeStyle {
    /// Just show a colored dot
    Dot,
    /// Show count number
    Count,
    /// Show count with icon
    IconCount,
}

/// File icon decoration for diagnostics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconDecoration {
    /// No decoration
    None,
    /// Error indicator (red)
    Error,
    /// Warning indicator (yellow)
    Warning,
    /// Both error and warning
    Mixed,
}

impl IconDecoration {
    /// Create from counts
    pub fn from_counts(counts: &DiagnosticCounts) -> Self {
        if counts.errors > 0 && counts.warnings > 0 {
            Self::Mixed
        } else if counts.errors > 0 {
            Self::Error
        } else if counts.warnings > 0 {
            Self::Warning
        } else {
            Self::None
        }
    }
}
