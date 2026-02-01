//! Infinite scroll component

use gpui::*;
use gpui::prelude::*;

/// Infinite scroll container with loading indicator
#[derive(IntoElement)]
pub struct InfiniteScroll {
    content: Div,
    loading: bool,
    has_more: bool,
    loading_text: SharedString,
    end_text: SharedString,
    height: f32,
    text_color: Option<Hsla>,
}

impl InfiniteScroll {
    pub fn new() -> Self {
        Self {
            content: div(),
            loading: false,
            has_more: true,
            loading_text: "Loading more...".into(),
            end_text: "No more items".into(),
            height: 400.0,
            text_color: None,
        }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.content = div().child(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.content = div().children(children);
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn has_more(mut self, has_more: bool) -> Self {
        self.has_more = has_more;
        self
    }

    pub fn loading_text(mut self, text: impl Into<SharedString>) -> Self {
        self.loading_text = text.into();
        self
    }

    pub fn end_text(mut self, text: impl Into<SharedString>) -> Self {
        self.end_text = text.into();
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn text_color(mut self, color: Hsla) -> Self {
        self.text_color = Some(color);
        self
    }
}

impl Default for InfiniteScroll {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for InfiniteScroll {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text_color = self.text_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.5,
            a: 1.0,
        });

        let footer = if self.loading {
            div()
                .flex()
                .items_center()
                .justify_center()
                .py_4()
                .gap_2()
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(text_color)
                        .child("⟳"),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(text_color)
                        .child(self.loading_text),
                )
        } else if !self.has_more {
            div()
                .flex()
                .items_center()
                .justify_center()
                .py_4()
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(text_color)
                        .child(self.end_text),
                )
        } else {
            div()
        };

        div()
            .h(px(self.height))
            .id("scroll-infinite-scroll")
            .overflow_y_scroll()
            .flex()
            .flex_col()
            .child(self.content)
            .child(footer)
    }
}
