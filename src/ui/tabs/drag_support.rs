use gpui::*;

use super::tab_bar::TabBar;
use super::types::*;

impl TabBar {
    /// Handle drag start
    pub(super) fn start_drag(&mut self, index: usize, cx: &mut Context<Self>) {
        self.dragging_index = Some(index);
        cx.notify();
    }

    /// Handle drag over
    pub(super) fn handle_drag_over(&mut self, target_index: usize, cx: &mut Context<Self>) {
        if let Some(from) = self.dragging_index {
            if from != target_index {
                // Reorder tabs
                let tab = self.tabs.remove(from);
                let insert_at = if target_index > from {
                    target_index
                } else {
                    target_index
                };
                self.tabs.insert(insert_at.min(self.tabs.len()), tab);

                // Update active index if needed
                if self.active_index == from {
                    self.active_index = insert_at.min(self.tabs.len() - 1);
                } else if from < self.active_index && target_index >= self.active_index {
                    self.active_index -= 1;
                } else if from > self.active_index && target_index <= self.active_index {
                    self.active_index += 1;
                }

                self.dragging_index = Some(insert_at.min(self.tabs.len() - 1));
                cx.emit(TabBarEvent::TabsReordered {
                    from,
                    to: insert_at.min(self.tabs.len() - 1),
                });
                cx.notify();
            }
        }
    }

    /// Handle drag end
    pub(super) fn end_drag(&mut self, cx: &mut Context<Self>) {
        self.dragging_index = None;
        cx.notify();
    }
}
