//! Page size selector component

use gpui::*;
use gpui::prelude::*;

/// Page size selector
#[derive(Clone)]
pub struct PageSizeSelector {
    current: usize,
    options: Vec<usize>,
}

impl PageSizeSelector {
    pub fn new(current: usize) -> Self {
        Self {
            current,
            options: vec![10, 25, 50, 100],
        }
    }

    pub fn options(mut self, options: Vec<usize>) -> Self {
        self.options = options;
        self
    }
}

impl RenderOnce for PageSizeSelector {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        div()
            .flex()
            .items_center()
            .gap_2()
            .child(
                div()
                    .text_sm()
                    .text_color(text_muted)
                    .child("Show:")
            )
            .child(
                div()
                    .h(px(28.0))
                    .px_2()
                    .rounded(px(6.0))
                    .border_1()
                    .border_color(border)
                    .bg(surface)
                    .flex()
                    .items_center()
                    .text_sm()
                    .text_color(text)
                    .child(self.current.to_string())
            )
            .child(
                div()
                    .text_sm()
                    .text_color(text_muted)
                    .child("per page")
            )
    }
}
