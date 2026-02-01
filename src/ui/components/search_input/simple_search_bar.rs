//! Simple stateless search bar component

use gpui::prelude::*;
use gpui::*;

/// Simple stateless search bar
#[derive(Clone)]
pub struct SimpleSearchBar {
    placeholder: String,
    query: String,
}

impl SimpleSearchBar {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            query: String::new(),
        }
    }

    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.query = query.into();
        self
    }
}

impl RenderOnce for SimpleSearchBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let border = hsla(0.0, 0.0, 0.3, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let surface = hsla(0.0, 0.0, 0.15, 1.0);

        let has_query = !self.query.is_empty();

        div()
            .h(px(36.0))
            .w_full()
            .px_3()
            .rounded(px(6.0))
            .border_1()
            .border_color(border)
            .bg(surface)
            .flex()
            .items_center()
            .gap_2()
            .child(div().text_color(text_muted).child("🔍"))
            .child(
                div()
                    .flex_1()
                    .text_sm()
                    .when(has_query, |d| d.text_color(text).child(self.query))
                    .when(!has_query, |d| {
                        d.text_color(text_muted).child(self.placeholder)
                    }),
            )
    }
}
