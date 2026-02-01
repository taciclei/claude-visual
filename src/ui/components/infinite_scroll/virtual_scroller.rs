//! Virtual scroller for large lists

use gpui::*;
use gpui::prelude::*;

/// Virtual scroller for large lists
#[derive(IntoElement)]
pub struct VirtualScroller {
    id: ElementId,
    item_height: f32,
    total_items: usize,
    visible_items: usize,
    scroll_offset: f32,
    overscan: usize,
    buffer_size: usize,
    background: gpui::Hsla,
}

impl VirtualScroller {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            item_height: 48.0,
            total_items: 0,
            visible_items: 10,
            scroll_offset: 0.0,
            overscan: 3,
            buffer_size: 5,
            background: rgba(0x00000000).into(),
        }
    }

    pub fn item_height(mut self, height: f32) -> Self {
        self.item_height = height;
        self
    }

    pub fn total_items(mut self, count: usize) -> Self {
        self.total_items = count;
        self
    }

    pub fn visible_items(mut self, count: usize) -> Self {
        self.visible_items = count;
        self
    }

    pub fn scroll_offset(mut self, offset: f32) -> Self {
        self.scroll_offset = offset;
        self
    }

    pub fn overscan(mut self, count: usize) -> Self {
        self.overscan = count;
        self
    }

    pub fn buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = color;
        self
    }

    /// Calculate the range of items to render
    pub fn visible_range(&self) -> (usize, usize) {
        let start_index = (self.scroll_offset / self.item_height).floor() as usize;
        let start = start_index.saturating_sub(self.overscan);
        let end = (start_index + self.visible_items + self.overscan * 2).min(self.total_items);
        (start, end)
    }

    /// Get the total scrollable height
    pub fn total_height(&self) -> f32 {
        self.total_items as f32 * self.item_height
    }
}

impl RenderOnce for VirtualScroller {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let total_height = self.total_height();
        let (start, end) = self.visible_range();
        let offset_y = start as f32 * self.item_height;

        div()
            .id(self.id)
            .relative()
            .size_full()
            .overflow_y_scroll()
            .bg(self.background)
            .child(
                // Spacer for total scroll height
                div().h(px(total_height)).w_full().child(
                    // Visible items container
                    div()
                        .absolute()
                        .top(px(offset_y))
                        .left_0()
                        .right_0()
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgba(0x888888ff))
                                .child(format!("Rendering items {} to {} of {}", start, end, self.total_items)),
                        ),
                ),
            )
    }
}
