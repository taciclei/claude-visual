//! Variable display component

use gpui::prelude::*;
use gpui::*;

/// Variable display (like a variable name or constant)
#[derive(Clone)]
pub struct Variable {
    name: String,
    value: Option<String>,
}

impl Variable {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: None,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
}

impl RenderOnce for Variable {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bg = hsla(0.0, 0.0, 0.15, 1.0);
        let name_color = hsla(0.55, 0.7, 0.65, 1.0); // Blue for variable names
        let value_color = hsla(0.12, 0.7, 0.6, 1.0); // Orange for values
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);

        div()
            .px_2()
            .py_1()
            .bg(bg)
            .rounded(px(4.0))
            .flex()
            .items_center()
            .gap_1()
            .font_family("monospace")
            .text_sm()
            .child(div().text_color(name_color).child(self.name))
            .when_some(self.value, |d, val| {
                d.child(div().text_color(text_muted).child("="))
                    .child(div().text_color(value_color).child(val))
            })
    }
}
