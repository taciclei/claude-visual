//! Key-value pair display component

use gpui::prelude::*;
use gpui::*;

/// Key-value pair display
#[derive(Clone)]
pub struct KeyValue {
    key: String,
    value: String,
    inline: bool,
}

impl KeyValue {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            inline: true,
        }
    }

    pub fn stacked(mut self) -> Self {
        self.inline = false;
        self
    }
}

impl RenderOnce for KeyValue {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bg = hsla(0.0, 0.0, 0.12, 1.0);
        let key_color = hsla(0.0, 0.0, 0.6, 1.0);
        let value_color = hsla(0.0, 0.0, 0.9, 1.0);

        let mut container = div().px_2().py_1().bg(bg).rounded(px(4.0));

        if self.inline {
            container = container
                .flex()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(key_color)
                        .child(self.key),
                )
                .child(div().text_sm().text_color(value_color).child(self.value));
        } else {
            container = container
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(key_color)
                        .child(self.key),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(value_color)
                        .font_family("monospace")
                        .child(self.value),
                );
        }

        container
    }
}
