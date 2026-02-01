//! WordReveal component - word-by-word reveal

use gpui::prelude::*;
use gpui::*;

/// Word-by-word reveal
#[derive(IntoElement)]
pub struct WordReveal {
    id: ElementId,
    pub(crate) words: Vec<SharedString>,
    revealed_words: usize,
    word_delay: u32,
    font_size: f32,
    text_color: gpui::Hsla,
    hidden_color: gpui::Hsla,
}

impl WordReveal {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        let text = text.into();
        let words: Vec<SharedString> = text
            .split_whitespace()
            .map(|s| SharedString::from(s.to_string()))
            .collect();

        Self {
            id: id.into(),
            words,
            revealed_words: 0,
            word_delay: 100,
            font_size: 16.0,
            text_color: rgba(0xffffffff).into(),
            hidden_color: rgba(0xffffff00).into(),
        }
    }

    pub fn revealed_words(mut self, count: usize) -> Self {
        self.revealed_words = count;
        self
    }

    pub fn word_delay(mut self, ms: u32) -> Self {
        self.word_delay = ms;
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

    pub fn hidden_color(mut self, color: gpui::Hsla) -> Self {
        self.hidden_color = color;
        self
    }

    /// Check if reveal is complete
    pub fn is_complete(&self) -> bool {
        self.revealed_words >= self.words.len()
    }
}

impl RenderOnce for WordReveal {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .flex_wrap()
            .gap_1()
            .text_size(px(self.font_size))
            .children(self.words.iter().enumerate().map(|(i, word)| {
                let is_revealed = i < self.revealed_words;
                let color = if is_revealed {
                    self.text_color
                } else {
                    self.hidden_color
                };
                div().text_color(color).child(word.clone())
            }))
    }
}
