//! CodeBlockView struct and core implementation

use gpui::*;
use std::sync::Arc;

use crate::app::state::AppState;

use super::types::*;

mod actions;
mod core;
mod diff;
mod highlights;
mod search;

pub use actions::*;
pub use core::*;
pub use diff::*;
pub use highlights::*;
pub use search::*;

/// A stateful code block that can be collapsed
pub struct CodeBlockView {
    pub(crate) app_state: Arc<AppState>,
    pub(crate) code: String,
    pub(crate) language: Option<String>,
    pub(crate) collapsed: bool,
    pub(crate) show_line_numbers: bool,
    /// Search state
    pub(crate) search_visible: bool,
    pub(crate) search_query: String,
    pub(crate) search_matches: Vec<SearchMatch>,
    pub(crate) current_match_index: Option<usize>,
    pub(crate) focus_handle: FocusHandle,
    /// Display mode (normal or diff)
    pub(crate) display_mode: CodeDisplayMode,
    /// Parsed diff lines (when in diff mode)
    pub(crate) diff_lines: Vec<DiffLine>,
    /// Old code (for comparison in diff mode)
    pub(crate) old_code: Option<String>,
    /// Highlighted line ranges for references
    pub(crate) highlighted_ranges: Vec<HighlightedRange>,
    /// Whether "Copied!" feedback is showing
    pub(crate) show_copied_feedback: bool,
}

impl EventEmitter<CodeBlockEvent> for CodeBlockView {}

impl Focusable for CodeBlockView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl CodeBlockView {
    pub fn new(
        code: String,
        language: Option<String>,
        app_state: Arc<AppState>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            app_state,
            code,
            language,
            collapsed: false,
            show_line_numbers: true,
            search_visible: false,
            search_query: String::new(),
            search_matches: Vec::new(),
            current_match_index: None,
            focus_handle: cx.focus_handle(),
            display_mode: CodeDisplayMode::Normal,
            diff_lines: Vec::new(),
            old_code: None,
            highlighted_ranges: Vec::new(),
            show_copied_feedback: false,
        }
    }

    /// Create a code block with diff view between old and new code
    pub fn with_diff(
        old_code: String,
        new_code: String,
        language: Option<String>,
        app_state: Arc<AppState>,
        cx: &mut Context<Self>,
    ) -> Self {
        let diff_lines = Self::compute_diff(&old_code, &new_code);
        Self {
            app_state,
            code: new_code,
            language,
            collapsed: false,
            show_line_numbers: true,
            search_visible: false,
            search_query: String::new(),
            search_matches: Vec::new(),
            current_match_index: None,
            focus_handle: cx.focus_handle(),
            display_mode: CodeDisplayMode::Diff,
            diff_lines,
            old_code: Some(old_code),
            highlighted_ranges: Vec::new(),
            show_copied_feedback: false,
        }
    }

    /// Create a code block with highlighted lines
    pub fn with_highlights(
        code: String,
        language: Option<String>,
        highlights: Vec<HighlightedRange>,
        app_state: Arc<AppState>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            app_state,
            code,
            language,
            collapsed: false,
            show_line_numbers: true,
            search_visible: false,
            search_query: String::new(),
            search_matches: Vec::new(),
            current_match_index: None,
            focus_handle: cx.focus_handle(),
            display_mode: CodeDisplayMode::Normal,
            diff_lines: Vec::new(),
            old_code: None,
            highlighted_ranges: highlights,
            show_copied_feedback: false,
        }
    }

    /// Get the language name for display
    pub(crate) fn language_display(&self) -> String {
        self.language
            .as_ref()
            .map(|l| l.to_uppercase())
            .unwrap_or_else(|| "TEXT".to_string())
    }

    /// Check if the language is executable (shell/bash)
    pub(crate) fn is_executable(&self) -> bool {
        self.language
            .as_ref()
            .map(|l| {
                matches!(
                    l.to_lowercase().as_str(),
                    "bash" | "sh" | "shell" | "zsh" | "fish"
                )
            })
            .unwrap_or(false)
    }
}
