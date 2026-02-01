//! Simple details/summary component

use gpui::*;
use gpui::prelude::*;

/// Simple details/summary component
#[derive(IntoElement)]
pub struct Details {
    summary: SharedString,
    content: Div,
    open: bool,
    summary_color: Option<Hsla>,
    content_color: Option<Hsla>,
    border_bottom: bool,
}

impl Details {
    pub fn new(summary: impl Into<SharedString>) -> Self {
        Self {
            summary: summary.into(),
            content: div(),
            open: false,
            summary_color: None,
            content_color: None,
            border_bottom: false,
        }
    }

    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = div().child(content);
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn summary_color(mut self, color: Hsla) -> Self {
        self.summary_color = Some(color);
        self
    }

    pub fn content_color(mut self, color: Hsla) -> Self {
        self.content_color = Some(color);
        self
    }

    pub fn border_bottom(mut self, border: bool) -> Self {
        self.border_bottom = border;
        self
    }
}

impl RenderOnce for Details {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let summary_color = self.summary_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.9,
            a: 1.0,
        });
        let content_color = self.content_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.7,
            a: 1.0,
        });

        let chevron = if self.open { "▼" } else { "▶" };

        let mut container = div().flex().flex_col().w_full();

        if self.border_bottom {
            container = container.border_b_1().border_color(Hsla {
                h: 0.0,
                s: 0.0,
                l: 0.3,
                a: 1.0,
            });
        }

        let summary = div()
            .flex()
            .items_center()
            .gap_2()
            .py_2()
            .cursor_pointer()
            .child(
                div()
                    .text_size(px(10.0))
                    .text_color(Hsla {
                        h: 0.0,
                        s: 0.0,
                        l: 0.6,
                        a: 1.0,
                    })
                    .child(chevron),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(summary_color)
                    .font_weight(gpui::FontWeight::MEDIUM)
                    .child(self.summary),
            );

        let content = if self.open {
            div()
                .pl_5()
                .pb_2()
                .text_size(px(14.0))
                .text_color(content_color)
                .child(self.content)
        } else {
            div()
        };

        container.child(summary).child(content)
    }
}
