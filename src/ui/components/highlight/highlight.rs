//! Highlighted text span component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Highlighted text span
#[derive(Clone)]
pub struct Highlight {
    pub(crate) text: String,
    pub(crate) color: HighlightColor,
    pub(crate) style: TextHighlightStyle,
}

impl Highlight {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: HighlightColor::default(),
            style: TextHighlightStyle::default(),
        }
    }

    pub fn color(mut self, color: HighlightColor) -> Self {
        self.color = color;
        self
    }

    pub fn style(mut self, style: TextHighlightStyle) -> Self {
        self.style = style;
        self
    }

    pub fn yellow(text: impl Into<String>) -> Self {
        Self::new(text).color(HighlightColor::Yellow)
    }

    pub fn green(text: impl Into<String>) -> Self {
        Self::new(text).color(HighlightColor::Green)
    }

    pub fn blue(text: impl Into<String>) -> Self {
        Self::new(text).color(HighlightColor::Blue)
    }

    pub fn pink(text: impl Into<String>) -> Self {
        Self::new(text).color(HighlightColor::Pink)
    }

    pub fn orange(text: impl Into<String>) -> Self {
        Self::new(text).color(HighlightColor::Orange)
    }

    pub fn purple(text: impl Into<String>) -> Self {
        Self::new(text).color(HighlightColor::Purple)
    }
}

impl RenderOnce for Highlight {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text_color = hsla(0.0, 0.0, 0.9, 1.0);
        let bg = self.color.background();
        let border_color = self.color.border();

        let mut el = div().text_color(text_color).child(self.text);

        match self.style {
            TextHighlightStyle::Background => {
                el = el.px_1().rounded(px(2.0)).bg(bg);
            }
            TextHighlightStyle::Underline => {
                el = el.border_b_2().border_color(border_color);
            }
            TextHighlightStyle::Border => {
                el = el
                    .px_1()
                    .border_1()
                    .border_color(border_color)
                    .rounded(px(2.0));
            }
            TextHighlightStyle::Glow => {
                el = el.px_1().rounded(px(2.0)).bg(bg).shadow_sm();
            }
        }

        el
    }
}
