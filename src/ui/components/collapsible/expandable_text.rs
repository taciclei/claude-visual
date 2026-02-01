//! Expandable text that shows "Show more" / "Show less"

use gpui::*;
use gpui::prelude::*;

/// Expandable text that shows "Show more" / "Show less"
#[derive(IntoElement)]
pub struct ExpandableText {
    text: SharedString,
    expanded: bool,
    max_lines: usize,
    show_more_label: SharedString,
    show_less_label: SharedString,
    text_color: Option<Hsla>,
    link_color: Option<Hsla>,
}

impl ExpandableText {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            expanded: false,
            max_lines: 3,
            show_more_label: "Show more".into(),
            show_less_label: "Show less".into(),
            text_color: None,
            link_color: None,
        }
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    pub fn max_lines(mut self, lines: usize) -> Self {
        self.max_lines = lines;
        self
    }

    pub fn show_more_label(mut self, label: impl Into<SharedString>) -> Self {
        self.show_more_label = label.into();
        self
    }

    pub fn show_less_label(mut self, label: impl Into<SharedString>) -> Self {
        self.show_less_label = label.into();
        self
    }

    pub fn text_color(mut self, color: Hsla) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn link_color(mut self, color: Hsla) -> Self {
        self.link_color = Some(color);
        self
    }
}

impl RenderOnce for ExpandableText {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text_color = self.text_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.8,
            a: 1.0,
        });
        let link_color = self.link_color.unwrap_or(Hsla {
            h: 0.58,
            s: 0.7,
            l: 0.6,
            a: 1.0,
        });

        let toggle_label = if self.expanded {
            self.show_less_label
        } else {
            self.show_more_label
        };

        let text_div = div()
            .text_size(px(14.0))
            .text_color(text_color)
            .line_height(px(20.0))
            .when(!self.expanded, |d| {
                d.max_h(px(20.0 * self.max_lines as f32))
                    .overflow_hidden()
            })
            .child(self.text);

        let toggle = div()
            .mt_1()
            .text_size(px(13.0))
            .text_color(link_color)
            .cursor_pointer()
            .child(toggle_label);

        div().flex().flex_col().child(text_div).child(toggle)
    }
}
