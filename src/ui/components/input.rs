//! Text input component

use gpui::*;
use gpui::prelude::*;
use gpui::prelude::*;
use gpui::prelude::*;

/// Text input component
pub struct TextInput {
    value: String,
    placeholder: String,
    disabled: bool,
    error: Option<String>,
}

impl TextInput {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            placeholder: String::new(),
            disabled: false,
            error: None,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn error(mut self, error: Option<String>) -> Self {
        self.error = error;
        self
    }
}

impl RenderOnce for TextInput {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let border_color = if self.error.is_some() {
            hsla(0.0, 0.84, 0.60, 1.0) // Error red
        } else {
            hsla(220.0 / 360.0, 0.13, 0.20, 1.0) // Default border
        };

        let text_color = if self.value.is_empty() {
            hsla(0.0, 0.0, 0.60, 1.0) // Muted
        } else {
            hsla(0.0, 0.0, 0.93, 1.0) // Primary text
        };

        div()
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .w_full()
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .bg(hsla(220.0 / 360.0, 0.13, 0.12, 1.0))
                    .border_1()
                    .border_color(border_color)
                    .text_sm()
                    .text_color(text_color)
                    .when(self.disabled, |d| d.opacity(0.5))
                    .child(if self.value.is_empty() {
                        self.placeholder.clone()
                    } else {
                        self.value.clone()
                    }),
            )
            .when_some(self.error, |this, error| {
                this.child(
                    div()
                        .text_xs()
                        .text_color(hsla(0.0, 0.84, 0.60, 1.0))
                        .child(error),
                )
            })
    }
}
