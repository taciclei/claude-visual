//! Bullet list component

use gpui::prelude::*;
use gpui::*;

/// Bulleted list
#[derive(Clone)]
pub struct BulletList {
    items: Vec<String>,
    bullet: String,
    nested: bool,
}

impl BulletList {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            bullet: "•".to_string(),
            nested: false,
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

    pub fn bullet(mut self, bullet: impl Into<String>) -> Self {
        self.bullet = bullet.into();
        self
    }

    pub fn nested(mut self) -> Self {
        self.nested = true;
        self
    }
}

impl Default for BulletList {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for BulletList {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);

        let indent = if self.nested { px(20.0) } else { px(0.0) };
        let bullet = self.bullet.clone();

        div()
            .pl(indent)
            .flex()
            .flex_col()
            .gap_2()
            .children(self.items.into_iter().map(move |item| {
                div()
                    .flex()
                    .items_start()
                    .gap_2()
                    .child(
                        div()
                            .w(px(16.0))
                            .flex_shrink_0()
                            .text_sm()
                            .text_color(text_muted)
                            .child(bullet.clone()),
                    )
                    .child(div().flex_1().text_sm().text_color(text).child(item))
            }))
    }
}
