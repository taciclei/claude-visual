//! Simple horizontal split layout component

use gpui::*;
use gpui::prelude::*;

/// Simple horizontal split layout
#[derive(Clone)]
pub struct HorizontalSplit {
    pub(crate) position: f32,
    pub(crate) divider_width: f32,
}

impl HorizontalSplit {
    pub fn new(position: f32) -> Self {
        Self {
            position: position.clamp(0.0, 1.0),
            divider_width: 4.0,
        }
    }

    pub fn divider_width(mut self, width: f32) -> Self {
        self.divider_width = width;
        self
    }
}

impl RenderOnce for HorizontalSplit {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let hover = hsla(0.6, 0.8, 0.6, 0.5);

        div()
            .w_full()
            .h_full()
            .flex()
            .flex_row()
            // Left pane
            .child(
                div()
                    .h_full()
                    .flex_1()
                    .overflow_hidden()
            )
            // Divider
            .child(
                div()
                    .w(px(self.divider_width))
                    .h_full()
                    .bg(border)
                    .flex_shrink_0()
                    .cursor(CursorStyle::ResizeLeftRight)
                    .hover(|s| s.bg(hover))
            )
            // Right pane
            .child(
                div()
                    .h_full()
                    .flex_1()
                    .overflow_hidden()
            )
    }
}
