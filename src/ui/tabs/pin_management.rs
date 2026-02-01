use gpui::*;

use super::tab_bar::TabBar;
use super::types::*;

impl TabBar {
    /// Pin a tab by index
    pub fn pin_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(tab) = self.tabs.get_mut(index) {
            if !tab.is_pinned {
                tab.is_pinned = true;
                // Move pinned tab to the front (after other pinned tabs)
                self.sort_tabs_by_pinned(cx);
            }
        }
    }

    /// Unpin a tab by index
    pub fn unpin_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(tab) = self.tabs.get_mut(index) {
            if tab.is_pinned {
                tab.is_pinned = false;
                self.sort_tabs_by_pinned(cx);
            }
        }
    }

    /// Toggle pin state of a tab
    pub fn toggle_pin(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(tab) = self.tabs.get(index) {
            if tab.is_pinned {
                self.unpin_tab(index, cx);
            } else {
                self.pin_tab(index, cx);
            }
        }
    }

    /// Sort tabs so pinned tabs come first
    fn sort_tabs_by_pinned(&mut self, cx: &mut Context<Self>) {
        // Remember the active tab's ID
        let active_id = self.tabs.get(self.active_index).map(|t| t.id.clone());

        // Partition: pinned first, then unpinned (stable sort to preserve order)
        self.tabs.sort_by_key(|tab| !tab.is_pinned);

        // Find the new index of the active tab
        if let Some(id) = active_id {
            if let Some(new_index) = self.tabs.iter().position(|t| t.id == id) {
                self.active_index = new_index;
            }
        }

        cx.notify();
    }

    /// Get count of pinned tabs
    pub fn pinned_count(&self) -> usize {
        self.tabs.iter().filter(|t| t.is_pinned).count()
    }

    /// Close all unpinned tabs
    pub fn close_all_unpinned(&mut self, cx: &mut Context<Self>) {
        // Keep only pinned tabs
        let active_id = self.tabs.get(self.active_index).map(|t| t.id.clone());
        self.tabs.retain(|tab| tab.is_pinned);

        // If no tabs left, create a new one
        if self.tabs.is_empty() {
            self.tabs.push(Tab::new());
        }

        // Update active index
        if let Some(id) = active_id {
            if let Some(new_index) = self.tabs.iter().position(|t| t.id == id) {
                self.active_index = new_index;
            } else {
                self.active_index = 0;
            }
        } else {
            self.active_index = 0;
        }

        cx.notify();
    }
}
