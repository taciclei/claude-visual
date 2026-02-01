//! Core diagnostics panel logic

use gpui::*;
use std::collections::HashMap;
use std::path::PathBuf;

use super::types::FileDiagnostics;
use crate::lsp::protocol::{Diagnostic, DiagnosticSeverity};

/// Diagnostics panel for displaying LSP diagnostics
pub struct DiagnosticsPanel {
    /// Diagnostics grouped by file
    pub(crate) files: HashMap<PathBuf, FileDiagnostics>,
    /// Order of files (for stable rendering)
    pub(crate) file_order: Vec<PathBuf>,
    /// Selected diagnostic index
    pub(crate) selected_index: Option<(PathBuf, usize)>,
    /// Filter by severity
    pub(crate) severity_filter: Option<DiagnosticSeverity>,
    /// Whether the panel is expanded
    pub(crate) is_expanded: bool,
}

impl DiagnosticsPanel {
    /// Create a new diagnostics panel
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            files: HashMap::new(),
            file_order: Vec::new(),
            selected_index: None,
            severity_filter: None,
            is_expanded: true,
        }
    }

    /// Update diagnostics for a file
    pub fn update_diagnostics(
        &mut self,
        path: PathBuf,
        diagnostics: Vec<Diagnostic>,
        cx: &mut Context<Self>,
    ) {
        if diagnostics.is_empty() {
            self.files.remove(&path);
            self.file_order.retain(|p| p != &path);
        } else {
            if !self.file_order.contains(&path) {
                self.file_order.push(path.clone());
            }
            self.files.insert(
                path.clone(),
                FileDiagnostics {
                    path,
                    diagnostics,
                    is_expanded: true,
                },
            );
        }
        cx.notify();
    }

    /// Clear all diagnostics
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.files.clear();
        self.file_order.clear();
        self.selected_index = None;
        cx.notify();
    }

    /// Toggle file expansion
    pub fn toggle_file(&mut self, path: &PathBuf, cx: &mut Context<Self>) {
        if let Some(file) = self.files.get_mut(path) {
            file.is_expanded = !file.is_expanded;
            cx.notify();
        }
    }

    /// Toggle panel expansion
    pub fn toggle_expanded(&mut self, cx: &mut Context<Self>) {
        self.is_expanded = !self.is_expanded;
        cx.notify();
    }

    /// Set severity filter
    pub fn set_filter(&mut self, severity: Option<DiagnosticSeverity>, cx: &mut Context<Self>) {
        self.severity_filter = severity;
        cx.notify();
    }

    /// Get total error count
    pub fn total_errors(&self) -> usize {
        self.files.values().map(|f| f.error_count()).sum()
    }

    /// Get total warning count
    pub fn total_warnings(&self) -> usize {
        self.files.values().map(|f| f.warning_count()).sum()
    }

    /// Get total diagnostic count
    pub fn total_count(&self) -> usize {
        self.files.values().map(|f| f.diagnostics.len()).sum()
    }

    /// Get filtered diagnostics for a file
    pub(crate) fn filtered_diagnostics<'a>(
        &'a self,
        file: &'a FileDiagnostics,
    ) -> Vec<&'a Diagnostic> {
        file.diagnostics
            .iter()
            .filter(|d| match self.severity_filter {
                Some(severity) => d.severity == Some(severity),
                None => true,
            })
            .collect()
    }
}
