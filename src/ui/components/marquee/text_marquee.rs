//! Text marquee - scrolling text

use gpui::prelude::*;
use gpui::*;

use super::types::*;

/// Text marquee - scrolling text
#[derive(IntoElement)]
pub struct TextMarquee {
    id: ElementId,
    text: SharedString,
    direction: MarqueeDirection,
    speed: MarqueeSpeed,
    font_size: f32,
    text_color: Option<gpui::Hsla>,
}

impl TextMarquee {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            direction: MarqueeDirection::default(),
            speed: MarqueeSpeed::default(),
            font_size: 14.0,
            text_color: None,
        }
    }

    pub fn direction(mut self, direction: MarqueeDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn speed(mut self, speed: MarqueeSpeed) -> Self {
        self.speed = speed;
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn text_color(mut self, color: gpui::Hsla) -> Self {
        self.text_color = Some(color);
        self
    }
}

impl RenderOnce for TextMarquee {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.text_color.unwrap_or(hsla(0.0, 0.0, 0.8, 1.0));

        div().id(self.id).w_full().overflow_hidden().child(
            div()
                .flex()
                .gap(px(40.0))
                .child(
                    div()
                        .text_size(px(self.font_size))
                        .text_color(color)
                        .whitespace_nowrap()
                        .child(self.text.clone()),
                )
                .child(
                    // Duplicate for seamless loop
                    div()
                        .text_size(px(self.font_size))
                        .text_color(color)
                        .whitespace_nowrap()
                        .child(self.text.clone()),
                ),
        )
    }
}
