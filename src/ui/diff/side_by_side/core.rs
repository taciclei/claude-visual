//! Core implementation of side-by-side diff view

use std::sync::Arc;
use gpui::*;

use crate::app::state::AppState;
use super::super::hunk::{DiffHunkManager, HunkAction};
use super::super::comments::DiffComments;
use super::types::{SideBySideDiffEvent, DiffDisplayMode};

/// Side-by-side diff view component
pub struct SideBySideDiffView {
    pub(crate) app_state: Arc<AppState>,
    /// Hunk manager with all hunks
    pub(crate) hunk_manager: DiffHunkManager,
    /// Inline comments
    pub(crate) comments: DiffComments,
    /// Display mode
    pub(crate) display_mode: DiffDisplayMode,
    /// File path
    pub(crate) file_path: String,
    /// Is collapsed
    pub(crate) collapsed: bool,
    /// Scroll offset
    pub(crate) scroll_offset: f32,
    /// Show line numbers
    pub(crate) show_line_numbers: bool,
    /// Show whitespace
    pub(crate) show_whitespace: bool,
    /// Syntax highlighting enabled
    pub(crate) syntax_highlighting: bool,
}

impl EventEmitter<SideBySideDiffEvent> for SideBySideDiffView {}

impl SideBySideDiffView {
    /// Create a new side-by-side diff view
    pub fn new(
        file_path: String,
        diff_text: &str,
        app_state: Arc<AppState>,
        _cx: &mut Context<Self>,
    ) -> Self {
        let mut hunk_manager = DiffHunkManager::new(std::path::PathBuf::from(&file_path));
        hunk_manager.parse_diff(diff_text);

        Self {
            app_state,
            hunk_manager,
            comments: DiffComments::default(),
            display_mode: DiffDisplayMode::SideBySide,
            file_path,
            collapsed: false,
            scroll_offset: 0.0,
            show_line_numbers: true,
            show_whitespace: false,
            syntax_highlighting: true,
        }
    }

    /// Toggle display mode
    pub fn toggle_mode(&mut self, cx: &mut Context<Self>) {
        self.display_mode = match self.display_mode {
            DiffDisplayMode::SideBySide => DiffDisplayMode::Unified,
            DiffDisplayMode::Unified => DiffDisplayMode::SideBySide,
        };
        cx.notify();
    }

    /// Toggle collapsed state
    pub fn toggle_collapsed(&mut self, cx: &mut Context<Self>) {
        self.collapsed = !self.collapsed;
        cx.notify();
    }

    /// Apply action to hunk
    pub fn apply_hunk_action(&mut self, hunk_id: usize, action: HunkAction, cx: &mut Context<Self>) {
        self.hunk_manager.apply_action(hunk_id, action.clone());
        cx.emit(SideBySideDiffEvent::HunkActionPerformed { hunk_id, action });
        cx.notify();
    }

    /// Apply all hunks
    pub fn apply_all(&mut self, cx: &mut Context<Self>) {
        self.hunk_manager.apply_all();
        cx.notify();
    }

    /// Reject all hunks
    pub fn reject_all(&mut self, cx: &mut Context<Self>) {
        self.hunk_manager.reject_all();
        cx.notify();
    }

    /// Reset all hunks
    pub fn reset_all(&mut self, cx: &mut Context<Self>) {
        self.hunk_manager.reset_all();
        cx.notify();
    }

    /// Navigate to next hunk
    pub fn next_hunk(&mut self, cx: &mut Context<Self>) {
        self.hunk_manager.next_hunk();
        cx.notify();
    }

    /// Navigate to previous hunk
    pub fn prev_hunk(&mut self, cx: &mut Context<Self>) {
        self.hunk_manager.prev_hunk();
        cx.notify();
    }

    /// Add comment at line
    pub fn add_comment(&mut self, hunk_id: usize, line_index: usize, side: &str, content: String, cx: &mut Context<Self>) {
        self.comments.add_comment(hunk_id, line_index, side, content);
        cx.emit(SideBySideDiffEvent::CommentAdded {
            hunk_id,
            line_index,
            side: side.to_string(),
        });
        cx.notify();
    }
}
