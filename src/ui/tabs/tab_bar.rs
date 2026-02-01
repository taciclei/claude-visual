use std::sync::Arc;

use gpui::*;

use crate::app::state::AppState;

use super::types::*;

/// Tab bar component for managing multiple conversation tabs
pub struct TabBar {
    pub(super) app_state: Arc<AppState>,
    pub(super) tabs: Vec<Tab>,
    pub(super) active_index: usize,
    pub(super) focus_handle: FocusHandle,
    /// Index being dragged, if any
    pub(super) dragging_index: Option<usize>,
    /// Whether the overflow menu is shown
    pub(super) show_overflow_menu: bool,
}

impl EventEmitter<TabBarEvent> for TabBar {}

impl TabBar {
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        // Start with a single empty tab
        let tabs = vec![Tab::new()];

        Self {
            app_state,
            tabs,
            active_index: 0,
            focus_handle: cx.focus_handle(),
            dragging_index: None,
            show_overflow_menu: false,
        }
    }

    /// Get current tabs
    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    /// Get the active tab index
    pub fn active_index(&self) -> usize {
        self.active_index
    }

    /// Get the active tab
    pub fn active_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.active_index)
    }

    /// Get mutable reference to active tab
    pub fn active_tab_mut(&mut self) -> Option<&mut Tab> {
        self.tabs.get_mut(self.active_index)
    }

    /// Get the focus handle
    pub fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
