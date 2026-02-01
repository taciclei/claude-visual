//! Split view component

use gpui::*;
use std::sync::Arc;

use super::node::SplitNode;
use super::types::{SplitDirection, SplitViewEvent};
use crate::app::state::AppState;

impl EventEmitter<SplitViewEvent> for SplitView {}

/// Main split view component
pub struct SplitView {
    pub(super) app_state: Arc<AppState>,
    /// Root of the split tree
    pub(super) root: SplitNode,
    /// Focus handle
    pub(super) focus_handle: FocusHandle,
    /// Maximum number of panes
    max_panes: usize,
}

impl SplitView {
    /// Create a new split view with a single pane
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        let mut root = SplitNode::leaf();
        root.set_focus(0);

        Self {
            app_state,
            root,
            focus_handle: cx.focus_handle(),
            max_panes: 4,
        }
    }

    /// Get the root node
    pub fn root(&self) -> &SplitNode {
        &self.root
    }

    /// Get current pane count
    pub fn pane_count(&self) -> usize {
        self.root.pane_count()
    }

    /// Get the focused pane index
    pub fn focused_pane(&self) -> Option<usize> {
        self.root.focused_index()
    }

    /// Focus a specific pane
    pub fn focus_pane(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.root.set_focus(index) {
            cx.emit(SplitViewEvent::PaneFocused(index));
            cx.notify();
        }
    }

    /// Focus the next pane
    pub fn focus_next(&mut self, cx: &mut Context<Self>) {
        if let Some(current) = self.focused_pane() {
            let next = (current + 1) % self.pane_count();
            self.focus_pane(next, cx);
        }
    }

    /// Focus the previous pane
    pub fn focus_prev(&mut self, cx: &mut Context<Self>) {
        if let Some(current) = self.focused_pane() {
            let count = self.pane_count();
            let prev = if current == 0 { count - 1 } else { current - 1 };
            self.focus_pane(prev, cx);
        }
    }

    /// Split the current view horizontally
    pub fn split_horizontal(&mut self, cx: &mut Context<Self>) {
        self.split(SplitDirection::Horizontal, cx);
    }

    /// Split the current view vertically
    pub fn split_vertical(&mut self, cx: &mut Context<Self>) {
        self.split(SplitDirection::Vertical, cx);
    }

    /// Split in a given direction
    fn split(&mut self, direction: SplitDirection, cx: &mut Context<Self>) {
        if self.pane_count() >= self.max_panes {
            tracing::info!("Maximum pane count ({}) reached", self.max_panes);
            return;
        }

        let focused_idx = match self.focused_pane() {
            Some(idx) => idx,
            None => return,
        };

        // Replace root with a split containing old root and new pane
        let old_root = std::mem::replace(&mut self.root, SplitNode::leaf());
        let new_pane = SplitNode::leaf();

        self.root = match direction {
            SplitDirection::Horizontal => SplitNode::horizontal(vec![old_root, new_pane]),
            SplitDirection::Vertical => SplitNode::vertical(vec![old_root, new_pane]),
        };

        // Keep focus on the original pane
        self.root.set_focus(focused_idx);

        cx.emit(SplitViewEvent::LayoutChanged);
        cx.notify();
    }

    /// Close the focused pane
    pub fn close_focused_pane(&mut self, cx: &mut Context<Self>) {
        if self.pane_count() <= 1 {
            tracing::info!("Cannot close the last pane");
            return;
        }

        let focused_idx = match self.focused_pane() {
            Some(idx) => idx,
            None => return,
        };

        // Simplified: reset to single pane
        self.root = SplitNode::leaf();
        self.root.set_focus(0);

        cx.emit(SplitViewEvent::PaneClosed(focused_idx));
        cx.notify();
    }

    /// Get focus handle
    pub fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
