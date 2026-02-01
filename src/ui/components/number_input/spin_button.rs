//! Spin button (compact stepper)

use gpui::prelude::*;
use gpui::*;

/// Spin button (compact stepper)
#[derive(Clone)]
pub struct SpinButton {
    value: i32,
    label: Option<String>,
}

impl SpinButton {
    pub fn new(value: i32) -> Self {
        Self { value, label: None }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl RenderOnce for SpinButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.2, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        div()
            .flex()
            .items_center()
            .gap_2()
            // Label
            .when_some(self.label, |d, label| {
                d.child(
                    div()
                        .text_sm()
                        .text_color(text_muted)
                        .child(label)
                )
            })
            // Spinner
            .child(
                div()
                    .flex()
                    .items_center()
                    .rounded(px(4.0))
                    .border_1()
                    .border_color(border)
                    .overflow_hidden()
                    // Value
                    .child(
                        div()
                            .w(px(40.0))
                            .h(px(24.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_sm()
                            .text_color(text)
                            .child(self.value.to_string())
                    )
                    // Buttons stack
                    .child(
                        div()
                            .w(px(16.0))
                            .h(px(24.0))
                            .flex()
                            .flex_col()
                            .border_l_1()
                            .border_color(border)
                            // Up
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_xs()
                                    .text_color(text_muted)
                                    .cursor_pointer()
                                    .hover(|s| s.bg(surface_hover))
                                    .child("▲")
                            )
                            // Divider
                            .child(
                                div()
                                    .h(px(1.0))
                                    .bg(border)
                            )
                            // Down
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_xs()
                                    .text_color(text_muted)
                                    .cursor_pointer()
                                    .hover(|s| s.bg(surface_hover))
                                    .child("▼")
                            )
                    )
            )
    }
}
