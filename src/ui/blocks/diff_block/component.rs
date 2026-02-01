//! DiffBlockView component definition and core methods

use std::sync::Arc;
use gpui::*;

use crate::app::state::AppState;
use super::parser::parse_diff;
use super::types::{DiffBlockEvent, DiffHunk};

/// A stateful diff visualization block
pub struct DiffBlockView {
    pub(super) app_state: Arc<AppState>,
    pub(super) file_path: String,
    pub(super) hunks: Vec<DiffHunk>,
    pub(super) collapsed: bool,
    pub(super) additions: usize,
    pub(super) deletions: usize,
}

impl EventEmitter<DiffBlockEvent> for DiffBlockView {}

impl DiffBlockView {
    pub fn new(file_path: String, diff_text: &str, app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        let (hunks, additions, deletions) = parse_diff(diff_text);

        Self {
            app_state,
            file_path,
            hunks,
            collapsed: false,
            additions,
            deletions,
        }
    }

    /// Parse diff text to create a DiffBlockView
    pub fn from_diff(diff_text: &str, app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        // Extract file path from diff header
        let file_path = diff_text
            .lines()
            .find(|l| l.starts_with("+++ "))
            .map(|l| l.trim_start_matches("+++ b/").to_string())
            .unwrap_or_else(|| "unknown".to_string());

        Self::new(file_path, diff_text, app_state, cx)
    }

    /// Toggle collapsed state
    pub fn toggle_collapsed(&mut self, cx: &mut Context<Self>) {
        self.collapsed = !self.collapsed;
        cx.notify();
    }

    /// Toggle a specific hunk's collapsed state
    pub fn toggle_hunk(&mut self, hunk_idx: usize, cx: &mut Context<Self>) {
        if let Some(hunk) = self.hunks.get_mut(hunk_idx) {
            hunk.collapsed = !hunk.collapsed;
            cx.notify();
        }
    }

    /// Get file path
    pub(crate) fn file_path(&self) -> &str {
        &self.file_path
    }
}
