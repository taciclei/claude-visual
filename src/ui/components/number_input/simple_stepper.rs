//! Simple stepper for stateless rendering

use gpui::prelude::*;
use gpui::*;

/// Simple stepper for stateless rendering
#[derive(Clone)]
pub struct SimpleStepper {
    value: i32,
    min: i32,
    max: i32,
}

impl SimpleStepper {
    pub fn new(value: i32, min: i32, max: i32) -> Self {
        Self { value, min, max }
    }
}

impl RenderOnce for SimpleStepper {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.2, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        let can_dec = self.value > self.min;
        let can_inc = self.value < self.max;

        div()
            .h(px(32.0))
            .flex()
            .items_center()
            .rounded(px(6.0))
            .border_1()
            .border_color(border)
            .overflow_hidden()
            // Decrement
            .child(
                div()
                    .w(px(28.0))
                    .h_full()
                    .bg(surface)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_sm()
                    .text_color(if can_dec { text } else { text_muted })
                    .when(can_dec, |d| {
                        d.cursor_pointer().hover(|s| s.bg(surface_hover))
                    })
                    .child("−"),
            )
            // Value
            .child(
                div()
                    .w(px(48.0))
                    .h_full()
                    .border_x_1()
                    .border_color(border)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_sm()
                    .text_color(text)
                    .child(self.value.to_string()),
            )
            // Increment
            .child(
                div()
                    .w(px(28.0))
                    .h_full()
                    .bg(surface)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_sm()
                    .text_color(if can_inc { text } else { text_muted })
                    .when(can_inc, |d| {
                        d.cursor_pointer().hover(|s| s.bg(surface_hover))
                    })
                    .child("+"),
            )
    }
}
