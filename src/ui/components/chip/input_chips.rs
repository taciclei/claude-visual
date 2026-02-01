//! Input chips component (for email-like inputs)

use gpui::prelude::*;
use gpui::*;

/// Input chips (for email-like inputs)
#[derive(Clone)]
pub struct InputChips {
    pub(crate) values: Vec<String>,
    pub(crate) placeholder: String,
}

impl InputChips {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            placeholder: "Add item...".to_string(),
        }
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.values.push(value.into());
        self
    }

    pub fn values(mut self, values: Vec<impl Into<String>>) -> Self {
        self.values = values.into_iter().map(|v| v.into()).collect();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
}

impl Default for InputChips {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for InputChips {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.2, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);

        div()
            .w_full()
            .min_h(px(44.0))
            .px_2()
            .py_1()
            .rounded(px(6.0))
            .border_1()
            .border_color(border)
            .bg(surface)
            .flex()
            .flex_wrap()
            .items_center()
            .gap_2()
            // Existing chips
            .children(self.values.into_iter().map(|value| {
                div()
                    .h(px(28.0))
                    .px_2()
                    .rounded_full()
                    .bg(accent.opacity(0.15))
                    .border_1()
                    .border_color(accent.opacity(0.3))
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(div().text_sm().text_color(text).child(value))
                    .child(
                        div()
                            .size(px(16.0))
                            .rounded_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_xs()
                            .text_color(text_muted)
                            .cursor_pointer()
                            .hover(|s| s.bg(surface_hover))
                            .child("×"),
                    )
            }))
            // Input placeholder
            .child(
                div()
                    .h(px(28.0))
                    .flex()
                    .items_center()
                    .text_sm()
                    .text_color(text_muted)
                    .child(self.placeholder),
            )
    }
}
