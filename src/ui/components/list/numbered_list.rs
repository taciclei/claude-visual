//! Numbered list component

use gpui::prelude::*;
use gpui::*;

/// Numbered list
#[derive(Clone)]
pub struct NumberedList {
    items: Vec<String>,
    start: usize,
}

impl NumberedList {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            start: 1,
        }
    }

    pub fn item(mut self, text: impl Into<String>) -> Self {
        self.items.push(text.into());
        self
    }

    pub fn items(mut self, items: Vec<impl Into<String>>) -> Self {
        self.items = items.into_iter().map(|i| i.into()).collect();
        self
    }

    pub fn start(mut self, num: usize) -> Self {
        self.start = num;
        self
    }
}

impl Default for NumberedList {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for NumberedList {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);

        let start = self.start;

        div()
            .flex()
            .flex_col()
            .gap_2()
            .children(self.items.into_iter().enumerate().map(move |(idx, item)| {
                div()
                    .flex()
                    .items_start()
                    .gap_2()
                    .child(
                        div()
                            .w(px(24.0))
                            .flex_shrink_0()
                            .text_sm()
                            .text_color(text_muted)
                            .child(format!("{}.", start + idx)),
                    )
                    .child(div().flex_1().text_sm().text_color(text).child(item))
            }))
    }
}
