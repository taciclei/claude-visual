//! Simple pagination display component

use gpui::prelude::*;
use gpui::*;

/// Simple pagination display
#[derive(Clone)]
pub struct SimplePagination {
    current: usize,
    total: usize,
    show_buttons: bool,
}

impl SimplePagination {
    pub fn new(current: usize, total: usize) -> Self {
        Self {
            current,
            total,
            show_buttons: true,
        }
    }

    pub fn hide_buttons(mut self) -> Self {
        self.show_buttons = false;
        self
    }
}

impl RenderOnce for SimplePagination {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.18, 1.0);

        let can_prev = self.current > 1;
        let can_next = self.current < self.total;

        div()
            .flex()
            .items_center()
            .gap_2()
            .when(self.show_buttons, |d| {
                d.child(
                    div()
                        .size(px(28.0))
                        .rounded(px(6.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_sm()
                        .text_color(if can_prev { text } else { text_muted })
                        .when(can_prev, |d| {
                            d.cursor_pointer().hover(|s| s.bg(surface_hover))
                        })
                        .child("‹"),
                )
            })
            .child(
                div()
                    .text_sm()
                    .text_color(text_muted)
                    .child(format!("{} / {}", self.current, self.total)),
            )
            .when(self.show_buttons, |d| {
                d.child(
                    div()
                        .size(px(28.0))
                        .rounded(px(6.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_sm()
                        .text_color(if can_next { text } else { text_muted })
                        .when(can_next, |d| {
                            d.cursor_pointer().hover(|s| s.bg(surface_hover))
                        })
                        .child("›"),
                )
            })
    }
}
