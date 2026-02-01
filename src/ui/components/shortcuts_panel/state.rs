//! Shortcuts panel state and logic

use super::types::*;
use crate::app::state::AppState;
use gpui::*;
use std::sync::Arc;

/// Keyboard shortcuts help panel
pub struct ShortcutsPanel {
    pub(super) app_state: Arc<AppState>,
    pub(super) focus_handle: FocusHandle,
    pub(super) search_query: String,
    pub(super) selected_group: usize,
}

impl ShortcutsPanel {
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            focus_handle: cx.focus_handle(),
            search_query: String::new(),
            selected_group: 0,
        }
    }

    /// Handle key input
    pub(super) fn handle_key_down(
        &mut self,
        event: &KeyDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event.keystroke.key.as_str() {
            "escape" => {
                cx.emit(ShortcutsPanelEvent::Dismissed);
            }
            "backspace" => {
                self.search_query.pop();
                cx.notify();
            }
            "up" => {
                if self.selected_group > 0 {
                    self.selected_group -= 1;
                } else {
                    self.selected_group = SHORTCUT_GROUPS.len() - 1;
                }
                cx.notify();
            }
            "down" => {
                if self.selected_group < SHORTCUT_GROUPS.len() - 1 {
                    self.selected_group += 1;
                } else {
                    self.selected_group = 0;
                }
                cx.notify();
            }
            _ => {}
        }
    }

    /// Handle text input for search
    pub(super) fn handle_input(
        &mut self,
        text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.search_query.push_str(text);
        cx.notify();
    }

    /// Get filtered shortcuts based on search query
    pub(super) fn filtered_groups(&self) -> Vec<(&'static ShortcutGroup, Vec<&'static Shortcut>)> {
        if self.search_query.is_empty() {
            SHORTCUT_GROUPS
                .iter()
                .map(|g| (g, g.shortcuts.iter().collect()))
                .collect()
        } else {
            let query = self.search_query.to_lowercase();
            SHORTCUT_GROUPS
                .iter()
                .filter_map(|group| {
                    let matching: Vec<_> = group
                        .shortcuts
                        .iter()
                        .filter(|s| {
                            s.keys.to_lowercase().contains(&query)
                                || s.description.to_lowercase().contains(&query)
                        })
                        .collect();

                    if matching.is_empty() {
                        None
                    } else {
                        Some((group, matching))
                    }
                })
                .collect()
        }
    }
}

impl EventEmitter<ShortcutsPanelEvent> for ShortcutsPanel {}

impl Focusable for ShortcutsPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
