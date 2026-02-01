//! Key-value table component

use gpui::*;
use gpui::prelude::*;

/// Key-value table for displaying properties
#[derive(Clone)]
pub struct KeyValueTable {
    items: Vec<(String, String)>,
    label_width: f32,
}

impl KeyValueTable {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            label_width: 120.0,
        }
    }

    pub fn item(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.items.push((key.into(), value.into()));
        self
    }

    pub fn label_width(mut self, width: f32) -> Self {
        self.label_width = width;
        self
    }
}

impl Default for KeyValueTable {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for KeyValueTable {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.6, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        div()
            .w_full()
            .flex()
            .flex_col()
            .children(self.items.into_iter().enumerate().map(|(idx, (key, value))| {
                div()
                    .w_full()
                    .py_2()
                    .flex()
                    .items_start()
                    .when(idx > 0, |d| d.border_t_1().border_color(border))
                    .child(
                        div()
                            .w(px(self.label_width))
                            .flex_shrink_0()
                            .text_sm()
                            .text_color(text_muted)
                            .child(key)
                    )
                    .child(
                        div()
                            .flex_1()
                            .text_sm()
                            .text_color(text)
                            .child(value)
                    )
            }))
    }
}
