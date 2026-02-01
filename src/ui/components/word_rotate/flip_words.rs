//! FlipWords component - flip animation between words

use gpui::*;
use gpui::prelude::*;

/// Flip words component - flip animation between words
#[derive(IntoElement)]
pub struct FlipWords {
    id: ElementId,
    words: Vec<SharedString>,
    current_index: usize,
    font_size: f32,
    text_color: Option<gpui::Hsla>,
}

impl FlipWords {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            words: Vec::new(),
            current_index: 0,
            font_size: 24.0,
            text_color: None,
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

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn text_color(mut self, color: gpui::Hsla) -> Self {
        self.text_color = Some(color);
        self
    }
}

impl RenderOnce for FlipWords {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.text_color.unwrap_or(hsla(0.6, 0.7, 0.5, 1.0));
        let current_word = self.words
            .get(self.current_index)
            .cloned()
            .unwrap_or("".into());

        div()
            .id(self.id)
            .overflow_hidden()
            .child(
                div()
                    .text_size(px(self.font_size))
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(color)
                    .child(current_word)
            )
    }
}
