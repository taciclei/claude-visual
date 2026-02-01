//! Virtual List Component
//!
//! Efficient rendering of large lists by only rendering visible items.
//! Critical for performance with large conversation histories.

use gpui::prelude::*;
use gpui::prelude::*;
use gpui::prelude::*;
use gpui::*;
use std::ops::Range;

/// Configuration for the virtual list
#[derive(Clone, Debug)]
pub struct VirtualListConfig {
    /// Fixed height for each item (if uniform)
    pub item_height: Pixels,
    /// Number of items to render above/below viewport (buffer)
    pub overscan: usize,
    /// Total number of items
    pub item_count: usize,
}

impl Default for VirtualListConfig {
    fn default() -> Self {
        Self {
            item_height: px(60.0),
            overscan: 5,
            item_count: 0,
        }
    }
}

/// State for virtual list scrolling
pub struct VirtualListState {
    /// Current scroll offset in pixels
    scroll_offset: Pixels,
    /// Viewport height
    viewport_height: Pixels,
    /// Configuration
    config: VirtualListConfig,
}

impl VirtualListState {
    /// Create new virtual list state
    pub fn new(config: VirtualListConfig) -> Self {
        Self {
            scroll_offset: px(0.0),
            viewport_height: px(600.0),
            config,
        }
    }

    /// Update scroll offset
    pub fn set_scroll_offset(&mut self, offset: Pixels) {
        self.scroll_offset = offset;
    }

    /// Update viewport height
    pub fn set_viewport_height(&mut self, height: Pixels) {
        self.viewport_height = height;
    }

    /// Update item count
    pub fn set_item_count(&mut self, count: usize) {
        self.config.item_count = count;
    }

    /// Get the range of visible item indices
    pub fn visible_range(&self) -> Range<usize> {
        if self.config.item_count == 0 {
            return 0..0;
        }

        let item_height: f32 = self.config.item_height.into();
        let scroll: f32 = self.scroll_offset.into();
        let viewport: f32 = self.viewport_height.into();
        let overscan = self.config.overscan;

        // Calculate first visible item
        let first_visible = (scroll / item_height).floor() as usize;
        let start = first_visible.saturating_sub(overscan);

        // Calculate last visible item
        let visible_count = (viewport / item_height).ceil() as usize;
        let end = (first_visible + visible_count + overscan).min(self.config.item_count);

        start..end
    }

    /// Get total content height
    pub fn total_height(&self) -> Pixels {
        let item_height: f32 = self.config.item_height.into();
        px(item_height * self.config.item_count as f32)
    }

    /// Get offset for an item at given index
    pub fn item_offset(&self, index: usize) -> Pixels {
        let item_height: f32 = self.config.item_height.into();
        px(item_height * index as f32)
    }

    /// Scroll to make an item visible
    pub fn scroll_to_item(&mut self, index: usize) {
        let item_top = self.item_offset(index);
        let item_top_f32: f32 = item_top.into();
        let item_height: f32 = self.config.item_height.into();
        let scroll_offset: f32 = self.scroll_offset.into();
        let viewport_height: f32 = self.viewport_height.into();
        let item_bottom = px(item_top_f32 + item_height);
        let item_bottom_f32: f32 = item_bottom.into();

        // If item is above viewport, scroll up
        if item_top_f32 < scroll_offset {
            self.scroll_offset = item_top;
        }
        // If item is below viewport, scroll down
        else if item_bottom_f32 > scroll_offset + viewport_height {
            self.scroll_offset = px(item_bottom_f32 - viewport_height);
        }
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self) {
        let total = self.total_height();
        let total_f32: f32 = total.into();
        let viewport_height: f32 = self.viewport_height.into();
        if total_f32 > viewport_height {
            self.scroll_offset = px(total_f32 - viewport_height);
        }
    }
}

/// Virtual list component for efficient list rendering
pub struct VirtualList<T: 'static + Clone> {
    /// List items
    items: Vec<T>,
    /// Virtual list state
    state: VirtualListState,
    /// Focus handle
    focus_handle: FocusHandle,
}

impl<T: 'static + Clone> VirtualList<T> {
    /// Create a new virtual list
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            items: Vec::new(),
            state: VirtualListState::new(VirtualListConfig::default()),
            focus_handle: cx.focus_handle(),
        }
    }

    /// Set items
    pub fn set_items(&mut self, items: Vec<T>, cx: &mut Context<Self>) {
        self.items = items;
        self.state.set_item_count(self.items.len());
        cx.notify();
    }

    /// Add an item
    pub fn push_item(&mut self, item: T, cx: &mut Context<Self>) {
        self.items.push(item);
        self.state.set_item_count(self.items.len());
        cx.notify();
    }

    /// Clear all items
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.items.clear();
        self.state.set_item_count(0);
        cx.notify();
    }

    /// Get visible items
    pub fn visible_items(&self) -> Vec<(usize, &T)> {
        let range = self.state.visible_range();
        self.items
            .iter()
            .enumerate()
            .filter(|(i, _)| range.contains(i))
            .collect()
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self, cx: &mut Context<Self>) {
        self.state.scroll_to_bottom();
        cx.notify();
    }

    /// Scroll to item
    pub fn scroll_to_item(&mut self, index: usize, cx: &mut Context<Self>) {
        self.state.scroll_to_item(index);
        cx.notify();
    }

    /// Configure item height
    pub fn set_item_height(&mut self, height: Pixels, cx: &mut Context<Self>) {
        self.state.config.item_height = height;
        cx.notify();
    }

    /// Configure overscan
    pub fn set_overscan(&mut self, overscan: usize, cx: &mut Context<Self>) {
        self.state.config.overscan = overscan;
        cx.notify();
    }

    /// Get state for rendering
    pub fn state(&self) -> &VirtualListState {
        &self.state
    }

    /// Handle scroll event
    pub fn on_scroll(&mut self, offset: Pixels, cx: &mut Context<Self>) {
        self.state.set_scroll_offset(offset);
        cx.notify();
    }
}

impl<T: 'static + Clone> Focusable for VirtualList<T> {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

/// Events emitted by VirtualList
pub enum VirtualListEvent {
    /// Item clicked
    ItemClicked(usize),
    /// Scroll position changed
    ScrollChanged(Pixels),
}

impl<T: 'static + Clone> EventEmitter<VirtualListEvent> for VirtualList<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visible_range() {
        let mut state = VirtualListState::new(VirtualListConfig {
            item_height: px(50.0),
            overscan: 2,
            item_count: 100,
        });
        state.set_viewport_height(px(200.0)); // 4 items visible
        state.set_scroll_offset(px(0.0));

        let range = state.visible_range();
        // 0-2 overscan + 4 visible + 2 overscan = 0..8 (clamped to 0..6)
        assert_eq!(range.start, 0);
        assert!(range.end >= 6);
    }

    #[test]
    fn test_scroll_to_item() {
        let mut state = VirtualListState::new(VirtualListConfig {
            item_height: px(50.0),
            overscan: 2,
            item_count: 100,
        });
        state.set_viewport_height(px(200.0));

        // Scroll to item 50 (should be at offset 2500)
        state.scroll_to_item(50);
        assert!(state.scroll_offset.0 > 0.0);
    }
}
