//! AnimatedText component - various text effects

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Animated text with various effects
#[derive(IntoElement)]
pub struct AnimatedText {
    id: ElementId,
    text: SharedString,
    pub(crate) effect: TextEffect,
    pub(crate) duration: u32,
    delay: u32,
    stagger: u32,
    font_size: f32,
    text_color: gpui::Hsla,
    highlight_color: Option<gpui::Hsla>,
}

impl AnimatedText {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            effect: TextEffect::default(),
            duration: 500,
            delay: 0,
            stagger: 50,
            font_size: 16.0,
            text_color: rgba(0xffffffff).into(),
            highlight_color: None,
        }
    }

    pub fn effect(mut self, effect: TextEffect) -> Self {
        self.effect = effect;
        self
    }

    pub fn duration(mut self, ms: u32) -> Self {
        self.duration = ms;
        self
    }

    pub fn delay(mut self, ms: u32) -> Self {
        self.delay = ms;
        self
    }

    pub fn stagger(mut self, ms: u32) -> Self {
        self.stagger = ms;
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn text_color(mut self, color: gpui::Hsla) -> Self {
        self.text_color = color;
        self
    }

    pub fn highlight_color(mut self, color: gpui::Hsla) -> Self {
        self.highlight_color = Some(color);
        self
    }
}

impl RenderOnce for AnimatedText {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .flex_wrap()
            .text_size(px(self.font_size))
            .text_color(self.text_color)
            .when(self.effect == TextEffect::Highlight, |d| {
                d.when_some(self.highlight_color, |d, color| d.bg(color))
            })
            .child(self.text.clone())
    }
}
