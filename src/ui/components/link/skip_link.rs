//! Skip link for accessibility

use gpui::*;
use gpui::prelude::*;

/// Skip link for accessibility
#[derive(IntoElement)]
pub struct SkipLink {
    label: SharedString,
    target: SharedString,
    background: Option<Hsla>,
    text_color: Option<Hsla>,
}

impl SkipLink {
    pub fn new(label: impl Into<SharedString>, target: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            target: target.into(),
            background: None,
            text_color: None,
        }
    }

    pub fn background(mut self, color: Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn text_color(mut self, color: Hsla) -> Self {
        self.text_color = Some(color);
        self
    }
}

impl RenderOnce for SkipLink {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let background = self.background.unwrap_or(Hsla {
            h: 0.58,
            s: 0.7,
            l: 0.5,
            a: 1.0,
        });
        let text_color = self.text_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 1.0,
            a: 1.0,
        });

        // Skip links are visually hidden until focused
        div()
            .absolute()
            .top(px(-100.0))
            .left_2()
            .bg(background)
            .text_color(text_color)
            .px_4()
            .py_2()
            .rounded_md()
            .text_size(px(14.0))
            .font_weight(gpui::FontWeight::MEDIUM)

            .focus(|s| s.top_2())
            .child(self.label)
    }
}
