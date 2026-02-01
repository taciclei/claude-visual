//! Split view functionality

use gpui::*;
use crate::ui::split::{SplitContainer, SplitContainerEvent, SplitDirection};
use super::core::Workspace;

impl Workspace {
    /// Toggle split view mode
    pub(in crate::ui::workspace) fn toggle_split_mode(&mut self, cx: &mut Context<Self>) {
        self.split_mode = !self.split_mode;

        if self.split_mode && self.split_container.is_none() {
            // Create split container
            let state = self.app_state.clone();
            let container = cx.new(|cx| SplitContainer::new(state, cx));

            // Subscribe to split container events
            cx.subscribe(&container, |this, _, event: &SplitContainerEvent, cx| {
                match event {
                    SplitContainerEvent::PaneFocused(idx) => {
                        tracing::info!("Split pane focused: {}", idx);
                        // Could sync with chat view focus
                    }
                    SplitContainerEvent::PaneClosed(idx) => {
                        tracing::info!("Split pane closed: {}", idx);
                    }
                    SplitContainerEvent::SplitRequested(dir) => {
                        tracing::info!("Split requested: {:?}", dir);
                    }
                }
            })
            .detach();

            self.split_container = Some(container);
        }

        cx.notify();
    }

    /// Split the current pane horizontally
    pub(in crate::ui::workspace) fn split_horizontal(&mut self, cx: &mut Context<Self>) {
        if !self.split_mode {
            self.toggle_split_mode(cx);
        }

        if let Some(container) = &self.split_container {
            container.update(cx, |c, cx| c.split_horizontal(cx));
        }
    }

    /// Split the current pane vertically
    pub(in crate::ui::workspace) fn split_vertical(&mut self, cx: &mut Context<Self>) {
        if !self.split_mode {
            self.toggle_split_mode(cx);
        }

        if let Some(container) = &self.split_container {
            container.update(cx, |c, cx| c.split_vertical(cx));
        }
    }

    /// Close the focused split pane
    pub(in crate::ui::workspace) fn close_split_pane(&mut self, cx: &mut Context<Self>) {
        if let Some(container) = &self.split_container {
            let pane_count = container.read(cx).pane_count();
            if pane_count <= 1 {
                // Exit split mode when closing last pane
                self.split_mode = false;
                self.split_container = None;
            } else {
                container.update(cx, |c, cx| c.close_focused_pane(cx));
            }
        }
        cx.notify();
    }
}
