//! LSP Status Bar
//!
//! Shows the status of connected language servers.

use gpui::*;
use std::collections::HashMap;

use crate::lsp::manager::Language;

mod render;
mod types;

pub use types::{LspServerStatus, LspStatusBarEvent};

/// LSP status bar component
pub struct LspStatusBar {
    /// Server statuses by language
    pub(crate) servers: HashMap<Language, LspServerStatus>,
    /// Whether the diagnostics panel is visible
    pub(crate) diagnostics_visible: bool,
}

impl LspStatusBar {
    /// Create a new LSP status bar
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            servers: HashMap::new(),
            diagnostics_visible: false,
        }
    }

    /// Update server status
    pub fn update_server(&mut self, status: LspServerStatus, cx: &mut Context<Self>) {
        self.servers.insert(status.language, status);
        cx.notify();
    }

    /// Remove server
    pub fn remove_server(&mut self, language: Language, cx: &mut Context<Self>) {
        self.servers.remove(&language);
        cx.notify();
    }

    /// Update error/warning counts for a language
    pub fn update_counts(
        &mut self,
        language: Language,
        errors: usize,
        warnings: usize,
        cx: &mut Context<Self>,
    ) {
        if let Some(status) = self.servers.get_mut(&language) {
            status.error_count = errors;
            status.warning_count = warnings;
            cx.notify();
        }
    }

    /// Set diagnostics panel visibility
    pub fn set_diagnostics_visible(&mut self, visible: bool, cx: &mut Context<Self>) {
        self.diagnostics_visible = visible;
        cx.notify();
    }

    /// Get total error count across all servers
    pub fn total_errors(&self) -> usize {
        self.servers.values().map(|s| s.error_count).sum()
    }

    /// Get total warning count across all servers
    pub fn total_warnings(&self) -> usize {
        self.servers.values().map(|s| s.warning_count).sum()
    }

    /// Get running server count
    pub fn running_count(&self) -> usize {
        self.servers.values().filter(|s| s.is_running).count()
    }

    /// Get language icon
    fn language_icon(language: Language) -> &'static str {
        match language {
            Language::Rust => "🦀",
            Language::TypeScript => "TS",
            Language::JavaScript => "JS",
            Language::Python => "🐍",
            Language::Go => "Go",
            Language::Json => "{}",
            Language::Toml => "⚙",
            Language::Markdown => "MD",
        }
    }
}

impl EventEmitter<LspStatusBarEvent> for LspStatusBar {}
