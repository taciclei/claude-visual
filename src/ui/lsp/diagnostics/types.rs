//! Type definitions for diagnostics panel

use gpui::*;
use std::path::PathBuf;

use crate::lsp::protocol::{Diagnostic, DiagnosticSeverity};

/// Simple color scheme for diagnostics panel
#[derive(Clone)]
pub(crate) struct SimpleColors {
    pub(crate) panel: Hsla,
    pub(crate) surface: Hsla,
    pub(crate) border: Hsla,
    pub(crate) hover: Hsla,
    pub(crate) selection: Hsla,
    pub(crate) text: Hsla,
    pub(crate) text_muted: Hsla,
    pub(crate) accent: Hsla,
    pub(crate) error: Hsla,
    pub(crate) warning: Hsla,
    pub(crate) info: Hsla,
    pub(crate) success: Hsla,
}

/// Default color scheme
pub(crate) fn default_colors() -> SimpleColors {
    SimpleColors {
        panel: hsla(220.0 / 360.0, 0.13, 0.10, 1.0),
        surface: hsla(220.0 / 360.0, 0.13, 0.12, 1.0),
        border: hsla(220.0 / 360.0, 0.13, 0.20, 1.0),
        hover: hsla(220.0 / 360.0, 0.13, 0.15, 1.0),
        selection: hsla(210.0 / 360.0, 0.50, 0.30, 1.0),
        text: hsla(0.0, 0.0, 0.93, 1.0),
        text_muted: hsla(0.0, 0.0, 0.60, 1.0),
        accent: hsla(210.0 / 360.0, 0.80, 0.55, 1.0),
        error: hsla(0.0, 0.84, 0.60, 1.0),
        warning: hsla(38.0 / 360.0, 0.92, 0.50, 1.0),
        info: hsla(200.0 / 360.0, 0.80, 0.60, 1.0),
        success: hsla(142.0 / 360.0, 0.71, 0.45, 1.0),
    }
}

/// Events emitted by the diagnostics panel
#[derive(Debug, Clone)]
pub enum DiagnosticsPanelEvent {
    /// Navigate to a diagnostic location
    GoToLocation {
        file: PathBuf,
        line: u32,
        column: u32,
    },
    /// Panel was closed
    Closed,
    /// Quick fix requested
    QuickFix(Diagnostic),
}

/// Group of diagnostics for a file
#[derive(Debug, Clone)]
pub struct FileDiagnostics {
    /// File path
    pub(crate) path: PathBuf,
    /// Diagnostics for this file
    pub(crate) diagnostics: Vec<Diagnostic>,
    /// Whether the file group is expanded
    pub(crate) is_expanded: bool,
}

impl FileDiagnostics {
    /// Count by severity
    pub fn count_by_severity(&self, severity: DiagnosticSeverity) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Some(severity))
            .count()
    }

    /// Error count
    pub fn error_count(&self) -> usize {
        self.count_by_severity(DiagnosticSeverity::Error)
    }

    /// Warning count
    pub fn warning_count(&self) -> usize {
        self.count_by_severity(DiagnosticSeverity::Warning)
    }
}

/// Get severity icon
pub(crate) fn severity_icon(severity: Option<DiagnosticSeverity>) -> &'static str {
    match severity {
        Some(DiagnosticSeverity::Error) => "✕",
        Some(DiagnosticSeverity::Warning) => "⚠",
        Some(DiagnosticSeverity::Information) => "ℹ",
        Some(DiagnosticSeverity::Hint) => "💡",
        None => "?",
    }
}

/// Get severity color
pub(crate) fn severity_color(severity: Option<DiagnosticSeverity>, colors: &SimpleColors) -> Hsla {
    match severity {
        Some(DiagnosticSeverity::Error) => colors.error,
        Some(DiagnosticSeverity::Warning) => colors.warning,
        Some(DiagnosticSeverity::Information) => colors.info,
        Some(DiagnosticSeverity::Hint) => colors.success,
        None => colors.text_muted,
    }
}
