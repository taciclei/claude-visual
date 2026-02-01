//! Search functionality for history sidebar

use gpui::*;

use super::core::HistorySidebar;
use super::types::DisplayMode;

impl HistorySidebar {
    /// Handle search input change
    pub(super) fn on_search_change(&mut self, text: &str, cx: &mut Context<Self>) {
        self.search_query = text.to_string();

        if text.trim().is_empty() {
            self.display_mode = DisplayMode::Recent;
            self.search_results.clear();
        } else {
            self.display_mode = DisplayMode::Search;
            self.perform_search(cx);
        }
        cx.notify();
    }

    /// Perform the search
    pub(super) fn perform_search(&mut self, _cx: &mut Context<Self>) {
        if self.search_query.trim().is_empty() {
            self.search_results.clear();
            return;
        }

        match self.app_state.database.search_messages_with_filter(
            &self.search_query,
            &self.search_filter,
            50,
        ) {
            Ok(results) => {
                self.search_results = results;
            }
            Err(e) => {
                tracing::error!("Search failed: {}", e);
                self.search_results.clear();
            }
        }
    }

    /// Clear search and return to recent view
    pub(super) fn clear_search(&mut self, cx: &mut Context<Self>) {
        self.search_query.clear();
        self.search_results.clear();
        self.display_mode = DisplayMode::Recent;
        cx.notify();
    }

    /// Handle keyboard input in search field
    pub(super) fn handle_search_key_down(&mut self, event: &KeyDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        match &event.keystroke.key {
            key if key == "escape" => {
                self.clear_search(cx);
            }
            key if key == "backspace" => {
                self.search_query.pop();
                self.on_search_change(&self.search_query.clone(), cx);
            }
            _ => {}
        }
    }

    /// Handle text input in search field
    pub(super) fn handle_search_input(&mut self, text: &str, _window: &mut Window, cx: &mut Context<Self>) {
        self.search_query.push_str(text);
        self.on_search_change(&self.search_query.clone(), cx);
    }
}
