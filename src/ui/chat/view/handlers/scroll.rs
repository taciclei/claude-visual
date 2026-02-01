//! Scroll event handlers

use gpui::*;
use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Mark as scrolled away from bottom (shows scroll-to-bottom button)
    pub fn on_scroll_away(&mut self, cx: &mut Context<Self>) {
        if !self.show_scroll_to_bottom {
            self.show_scroll_to_bottom = true;
            cx.notify();
        }
    }

    /// Scroll to bottom and reset state
    pub fn scroll_to_bottom(&mut self, cx: &mut Context<Self>) {
        self.show_scroll_to_bottom = false;
        self.unread_count = 0;
        cx.notify();
    }

    /// Scroll to and highlight the current search result
    pub fn scroll_to_search_result(&mut self, cx: &mut Context<Self>) {
        if let Some(result) = self.search.results.get(self.search.current_result) {
            let message_index = result.message_index;

            // Add to navigation history
            if let Some(current) = self.selected_message_index {
                if current != message_index {
                    // Truncate forward history if we're not at the end
                    if self.navigation_history_position < self.navigation_history.len() {
                        self.navigation_history.truncate(self.navigation_history_position);
                    }
                    self.navigation_history.push(current);
                    self.navigation_history_position = self.navigation_history.len();
                    // Limit history size
                    if self.navigation_history.len() > 50 {
                        self.navigation_history.remove(0);
                        self.navigation_history_position = self.navigation_history.len();
                    }
                }
            }

            // Select and highlight the message
            self.selected_message_index = Some(message_index);
            self.highlighted_message = Some((message_index, std::time::Instant::now()));
            self.update_message_selection_states(cx);

            // Disable auto-scroll to prevent jumping back
            self.auto_scroll = false;

            cx.notify();
        }
    }

    /// Toggle auto-scroll
    pub fn toggle_auto_scroll(&mut self, cx: &mut Context<Self>) {
        self.auto_scroll = !self.auto_scroll;
        cx.notify();
    }

    /// Set auto-scroll
    pub fn set_auto_scroll(&mut self, auto_scroll: bool, cx: &mut Context<Self>) {
        self.auto_scroll = auto_scroll;
        cx.notify();
    }

    /// Check if auto-scroll is enabled
    pub fn is_auto_scroll(&self) -> bool {
        self.auto_scroll
    }
}
