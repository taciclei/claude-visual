use gpui::*;

use super::tab_bar::TabBar;
use super::types::*;

impl TabBar {
    /// Add a new tab and make it active
    pub fn add_tab(&mut self, cx: &mut Context<Self>) {
        let tab = Tab::new();
        self.tabs.push(tab);
        self.active_index = self.tabs.len() - 1;
        cx.emit(TabBarEvent::NewTabRequested);
        cx.notify();
    }

    /// Close a tab by index
    pub fn close_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        // Don't close pinned tabs
        if let Some(tab) = self.tabs.get(index) {
            if tab.is_pinned {
                return;
            }
        }

        if self.tabs.len() <= 1 {
            // Don't close the last tab, just clear it
            if let Some(tab) = self.tabs.get_mut(0) {
                tab.title = "New Chat".to_string();
                tab.conversation_id = None;
                tab.is_dirty = false;
            }
            cx.emit(TabBarEvent::CloseTabRequested(index));
            cx.notify();
            return;
        }

        // Remove the tab
        if index < self.tabs.len() {
            self.tabs.remove(index);

            // Adjust active index
            if self.active_index >= self.tabs.len() {
                self.active_index = self.tabs.len() - 1;
            } else if self.active_index > index {
                self.active_index -= 1;
            }

            cx.emit(TabBarEvent::CloseTabRequested(index));
            cx.notify();
        }
    }

    /// Select a tab by index
    pub fn select_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.tabs.len() && index != self.active_index {
            self.active_index = index;
            cx.emit(TabBarEvent::TabSelected(index));
            cx.notify();
        }
    }

    /// Select the next tab
    pub fn select_next_tab(&mut self, cx: &mut Context<Self>) {
        if self.tabs.len() > 1 {
            let next = (self.active_index + 1) % self.tabs.len();
            self.select_tab(next, cx);
        }
    }

    /// Select the previous tab
    pub fn select_prev_tab(&mut self, cx: &mut Context<Self>) {
        if self.tabs.len() > 1 {
            let prev = if self.active_index == 0 {
                self.tabs.len() - 1
            } else {
                self.active_index - 1
            };
            self.select_tab(prev, cx);
        }
    }

    /// Select tab by number (1-9)
    pub fn select_tab_by_number(&mut self, num: usize, cx: &mut Context<Self>) {
        if num > 0 && num <= self.tabs.len() {
            self.select_tab(num - 1, cx);
        }
    }

    /// Update the title of the active tab
    pub fn update_active_title(&mut self, title: String, cx: &mut Context<Self>) {
        if let Some(tab) = self.tabs.get_mut(self.active_index) {
            // Truncate long titles
            tab.title = if title.len() > 30 {
                format!("{}...", &title[..27])
            } else {
                title
            };
            cx.notify();
        }
    }

    /// Set the conversation ID of the active tab
    pub fn set_active_conversation_id(&mut self, id: String, cx: &mut Context<Self>) {
        if let Some(tab) = self.tabs.get_mut(self.active_index) {
            tab.conversation_id = Some(id);
            cx.notify();
        }
    }

    /// Mark the active tab as dirty/clean
    pub fn set_active_dirty(&mut self, dirty: bool, cx: &mut Context<Self>) {
        if let Some(tab) = self.tabs.get_mut(self.active_index) {
            tab.is_dirty = dirty;
            cx.notify();
        }
    }
}
