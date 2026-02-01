//! Simple vertical split layout component

use gpui::prelude::*;
use gpui::*;

/// Simple vertical split layout
#[derive(Clone)]
pub struct VerticalSplit {
    pub(crate) position: f32,
    pub(crate) divider_height: f32,
}

impl VerticalSplit {
    pub fn new(position: f32) -> Self {
        Self {
            position: position.clamp(0.0, 1.0),
            divider_height: 4.0,
        }
    }

    pub fn divider_height(mut self, height: f32) -> Self {
        self.divider_height = height;
        self
    }
}

impl RenderOnce for VerticalSplit {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let hover = hsla(0.6, 0.8, 0.6, 0.5);

        div()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            // Top pane
            .child(div().w_full().flex_1().overflow_hidden())
            // Divider
            .child(
                div()
                    .h(px(self.divider_height))
                    .w_full()
                    .bg(border)
                    .flex_shrink_0()
                    .cursor(CursorStyle::ResizeUpDown)
                    .hover(|s| s.bg(hover)),
            )
            // Bottom pane
            .child(div().w_full().flex_1().overflow_hidden())
    }
}
