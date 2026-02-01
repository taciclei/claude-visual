//! CharacterReveal component - character-by-character reveal with effects

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Character-by-character reveal with effects
#[derive(IntoElement)]
pub struct CharacterReveal {
    id: ElementId,
    text: SharedString,
    revealed_count: usize,
    reveal_style: RevealStyle,
    char_delay: u32,
    font_size: f32,
    text_color: gpui::Hsla,
    hidden_color: gpui::Hsla,
}

impl CharacterReveal {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            revealed_count: 0,
            reveal_style: RevealStyle::default(),
            char_delay: 30,
            font_size: 16.0,
            text_color: rgba(0xffffffff).into(),
            hidden_color: rgba(0xffffff00).into(),
        }
    }

    pub fn revealed_count(mut self, count: usize) -> Self {
        self.revealed_count = count;
        self
    }

    pub fn reveal_style(mut self, style: RevealStyle) -> Self {
        self.reveal_style = style;
        self
    }

    pub fn char_delay(mut self, ms: u32) -> Self {
        self.char_delay = ms;
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
        self.revealed_count >= self.text.len()
    }

    /// Get progress as percentage
    pub fn progress(&self) -> f32 {
        if self.text.is_empty() {
            return 1.0;
        }
        self.revealed_count as f32 / self.text.len() as f32
    }
}

impl RenderOnce for CharacterReveal {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let revealed: String = self.text.chars().take(self.revealed_count).collect();
        let hidden: String = self.text.chars().skip(self.revealed_count).collect();

        div()
            .id(self.id)
            .flex()
            .text_size(px(self.font_size))
            .child(div().text_color(self.text_color).child(revealed))
            .child(div().text_color(self.hidden_color).child(hidden))
    }
}
