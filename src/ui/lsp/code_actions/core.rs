//! Core logic for code actions panel

use std::sync::Arc;

use gpui::*;

use crate::app::state::AppState;

use super::types::{CodeActionItem, CodeActionsEvent};

impl EventEmitter<CodeActionsEvent> for CodeActionsPanel {}

/// Code actions panel component
pub struct CodeActionsPanel {
    pub(crate) app_state: Arc<AppState>,
    pub(crate) actions: Vec<CodeActionItem>,
    pub(crate) selected_index: usize,
    pub(crate) filter_text: String,
    pub(crate) position: Option<Point<Pixels>>,
}

impl CodeActionsPanel {
    /// Create a new code actions panel
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            actions: Vec::new(),
            selected_index: 0,
            filter_text: String::new(),
            position: None,
        }
    }

    /// Set code actions
    pub fn set_actions(&mut self, actions: Vec<CodeActionItem>, cx: &mut Context<Self>) {
        self.actions = actions;
        self.selected_index = 0;

        // Sort: preferred first, then by kind
        self.actions.sort_by(|a, b| {
            match (a.is_preferred, b.is_preferred) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.kind.display_name().cmp(b.kind.display_name()),
            }
        });

        cx.notify();
    }

    /// Set position for the panel
    pub fn set_position(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        self.position = Some(position);
        cx.notify();
    }

    /// Clear actions
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.actions.clear();
        self.selected_index = 0;
        self.filter_text.clear();
        cx.notify();
    }

    /// Get filtered actions
    pub(crate) fn filtered_actions(&self) -> Vec<&CodeActionItem> {
        if self.filter_text.is_empty() {
            self.actions.iter().collect()
        } else {
            let filter = self.filter_text.to_lowercase();
            self.actions
                .iter()
                .filter(|a| a.title.to_lowercase().contains(&filter))
                .collect()
        }
    }

    /// Move selection up
    pub fn select_prev(&mut self, cx: &mut Context<Self>) {
        let count = self.filtered_actions().len();
        if count > 0 && self.selected_index > 0 {
            self.selected_index -= 1;
        }
        cx.notify();
    }

    /// Move selection down
    pub fn select_next(&mut self, cx: &mut Context<Self>) {
        let count = self.filtered_actions().len();
        if count > 0 && self.selected_index < count - 1 {
            self.selected_index += 1;
        }
        cx.notify();
    }

    /// Execute selected action
    pub fn execute_selected(&mut self, cx: &mut Context<Self>) {
        let actions = self.filtered_actions();
        if let Some(action) = actions.get(self.selected_index) {
            cx.emit(CodeActionsEvent::Execute(action.id));
        }
    }

    /// Check if panel has actions
    pub fn has_actions(&self) -> bool {
        !self.actions.is_empty()
    }

    /// Get action count
    pub fn action_count(&self) -> usize {
        self.actions.len()
    }
}
