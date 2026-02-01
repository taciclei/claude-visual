//! Typewriter component - types out text character by character

use gpui::*;
use gpui::prelude::*;

/// Typewriter effect component - types out text character by character
#[derive(IntoElement)]
pub struct Typewriter {
    id: ElementId,
    text: SharedString,
    visible_chars: usize,
    cursor_visible: bool,
    cursor_char: SharedString,
    font_size: f32,
    text_color: Option<gpui::Hsla>,
    cursor_color: Option<gpui::Hsla>,
}

impl Typewriter {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            visible_chars: 0,
            cursor_visible: true,
            cursor_char: "|".into(),
            font_size: 16.0,
            text_color: None,
            cursor_color: None,
        }
    }

    pub fn visible_chars(mut self, chars: usize) -> Self {
        self.visible_chars = chars;
        self
    }

    pub fn cursor_visible(mut self, visible: bool) -> Self {
        self.cursor_visible = visible;
        self
    }

    pub fn cursor_char(mut self, char: impl Into<SharedString>) -> Self {
        self.cursor_char = char.into();
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

    pub fn cursor_color(mut self, color: gpui::Hsla) -> Self {
        self.cursor_color = Some(color);
        self
    }
}

impl RenderOnce for Typewriter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.text_color.unwrap_or(hsla(0.0, 0.0, 0.9, 1.0));
        let cursor_color = self.cursor_color.unwrap_or(hsla(0.6, 0.7, 0.5, 1.0));

        let visible_text: String = self.text
            .chars()
            .take(self.visible_chars)
            .collect();

        div()
            .id(self.id)
            .flex()
            .child(
                div()
                    .text_size(px(self.font_size))
                    .text_color(color)
                    .child(visible_text)
            )
            .when(self.cursor_visible, |el| {
                el.child(
                    div()
                        .text_size(px(self.font_size))
                        .text_color(cursor_color)
                        .child(self.cursor_char.clone())
                )
            })
    }
}
