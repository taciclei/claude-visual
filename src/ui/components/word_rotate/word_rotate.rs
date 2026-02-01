//! WordRotate component - cycles through words with animation

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Word rotate component - cycles through words with animation
#[derive(IntoElement)]
pub struct WordRotate {
    id: ElementId,
    words: Vec<SharedString>,
    current_index: usize,
    animation: RotateAnimation,
    speed: RotateSpeed,
    font_size: f32,
    font_weight: gpui::FontWeight,
    text_color: Option<gpui::Hsla>,
    highlight_color: Option<gpui::Hsla>,
}

impl WordRotate {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            words: Vec::new(),
            current_index: 0,
            animation: RotateAnimation::default(),
            speed: RotateSpeed::default(),
            font_size: 24.0,
            font_weight: gpui::FontWeight::BOLD,
            text_color: None,
            highlight_color: None,
        }
    }

    pub fn words(mut self, words: Vec<impl Into<SharedString>>) -> Self {
        self.words = words.into_iter().map(|w| w.into()).collect();
        self
    }

    pub fn current_index(mut self, index: usize) -> Self {
        self.current_index = index;
        self
    }

    pub fn animation(mut self, animation: RotateAnimation) -> Self {
        self.animation = animation;
        self
    }

    pub fn speed(mut self, speed: RotateSpeed) -> Self {
        self.speed = speed;
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn font_weight(mut self, weight: gpui::FontWeight) -> Self {
        self.font_weight = weight;
        self
    }

    pub fn text_color(mut self, color: gpui::Hsla) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn highlight_color(mut self, color: gpui::Hsla) -> Self {
        self.highlight_color = Some(color);
        self
    }
}

impl RenderOnce for WordRotate {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.text_color.unwrap_or(hsla(0.0, 0.0, 0.95, 1.0));
        let current_word = self
            .words
            .get(self.current_index)
            .cloned()
            .unwrap_or("".into());

        div().id(self.id).relative().overflow_hidden().child(
            div()
                .text_size(px(self.font_size))
                .font_weight(self.font_weight)
                .text_color(color)
                .child(current_word),
        )
    }
}
