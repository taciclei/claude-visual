//! LSP Status Bar Types
//!
//! Type definitions for the LSP status bar.

use gpui::*;
use std::collections::HashMap;

use crate::lsp::manager::Language;

pub(crate) struct SimpleColors {
    pub surface: Hsla,
    pub border: Hsla,
    pub hover: Hsla,
    pub selection: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub error: Hsla,
    pub warning: Hsla,
    pub success: Hsla,
}

pub(crate) fn default_colors() -> SimpleColors {
    SimpleColors {
        surface: hsla(220.0 / 360.0, 0.13, 0.12, 1.0),
        border: hsla(220.0 / 360.0, 0.13, 0.20, 1.0),
        hover: hsla(220.0 / 360.0, 0.13, 0.15, 1.0),
        selection: hsla(210.0 / 360.0, 0.50, 0.30, 1.0),
        text: hsla(0.0, 0.0, 0.93, 1.0),
        text_muted: hsla(0.0, 0.0, 0.60, 1.0),
        error: hsla(0.0, 0.84, 0.60, 1.0),
        warning: hsla(38.0 / 360.0, 0.92, 0.50, 1.0),
        success: hsla(142.0 / 360.0, 0.71, 0.45, 1.0),
    }
}

/// Events emitted by the LSP status bar
#[derive(Debug, Clone)]
pub enum LspStatusBarEvent {
    /// Toggle diagnostics panel visibility
    ToggleDiagnostics,
    /// Server clicked
    ServerClicked(Language),
}

/// Server status for display
#[derive(Debug, Clone)]
pub struct LspServerStatus {
    /// Language
    pub language: Language,
    /// Whether the server is running
    pub is_running: bool,
    /// Error count for this language
    pub error_count: usize,
    /// Warning count for this language
    pub warning_count: usize,
    /// Last error message
    pub last_error: Option<String>,
}
