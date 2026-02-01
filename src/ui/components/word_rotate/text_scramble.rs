//! TextScramble component - scrambles text before revealing

use gpui::prelude::*;
use gpui::*;

/// Text scramble effect - scrambles text before revealing
#[derive(IntoElement)]
pub struct TextScramble {
    id: ElementId,
    text: SharedString,
    scramble_chars: SharedString,
    revealed_count: usize,
    font_size: f32,
    text_color: Option<gpui::Hsla>,
    scramble_color: Option<gpui::Hsla>,
}

impl TextScramble {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            scramble_chars: "!<>-_\\/[]{}—=+*^?#________".into(),
            revealed_count: 0,
            font_size: 16.0,
            text_color: None,
            scramble_color: None,
        }
    }

    pub fn scramble_chars(mut self, chars: impl Into<SharedString>) -> Self {
        self.scramble_chars = chars.into();
        self
    }

    pub fn revealed_count(mut self, count: usize) -> Self {
        self.revealed_count = count;
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

    pub fn scramble_color(mut self, color: gpui::Hsla) -> Self {
        self.scramble_color = Some(color);
        self
    }
}

impl RenderOnce for TextScramble {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.text_color.unwrap_or(hsla(0.0, 0.0, 0.9, 1.0));
        let scramble_color = self.scramble_color.unwrap_or(hsla(0.0, 0.0, 0.5, 1.0));

        let chars: Vec<char> = self.text.chars().collect();
        let scramble_chars: Vec<char> = self.scramble_chars.chars().collect();

        div()
            .id(self.id)
            .flex()
            .children(chars.into_iter().enumerate().map(|(i, c)| {
                let is_revealed = i < self.revealed_count;
                let display_char = if is_revealed || c == ' ' {
                    c
                } else {
                    // Use a "random" scramble character based on position
                    scramble_chars
                        .get(i % scramble_chars.len())
                        .copied()
                        .unwrap_or('_')
                };

                div()
                    .text_size(px(self.font_size))
                    .text_color(if is_revealed { color } else { scramble_color })
                    .child(display_char.to_string())
            }))
    }
}
