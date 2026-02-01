//! Decorative separator with pattern

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Decorative separator with pattern
#[derive(IntoElement)]
pub struct DecorativeSeparator {
    pattern: SeparatorPattern,
    color: Option<Hsla>,
    margin: f32,
}

impl DecorativeSeparator {
    pub fn new() -> Self {
        Self {
            pattern: SeparatorPattern::Dots,
            color: None,
            margin: 16.0,
        }
    }

    pub fn pattern(mut self, pattern: SeparatorPattern) -> Self {
        self.pattern = pattern;
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self
    }

    pub fn dots() -> Self {
        Self::new().pattern(SeparatorPattern::Dots)
    }

    pub fn stars() -> Self {
        Self::new().pattern(SeparatorPattern::Stars)
    }

    pub fn diamonds() -> Self {
        Self::new().pattern(SeparatorPattern::Diamonds)
    }
}

impl Default for DecorativeSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for DecorativeSeparator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.4,
            a: 1.0,
        });

        let pattern_char = match self.pattern {
            SeparatorPattern::Dots => "• • •",
            SeparatorPattern::Stars => "✦ ✦ ✦",
            SeparatorPattern::Diamonds => "◆ ◇ ◆",
            SeparatorPattern::Arrows => "» » »",
            SeparatorPattern::Wave => "~ ~ ~",
        };

        div()
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .my(px(self.margin))
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(color)
                    .child(pattern_char),
            )
    }
}
