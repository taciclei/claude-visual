//! TextScramble component - random characters before revealing

use gpui::*;
use gpui::prelude::*;

/// Text scramble effect (random characters before revealing)
#[derive(IntoElement)]
pub struct TextScramble {
    id: ElementId,
    target_text: SharedString,
    current_text: SharedString,
    scramble_chars: SharedString,
    progress: f32,
    font_size: f32,
    text_color: gpui::Hsla,
    scramble_color: gpui::Hsla,
}

impl TextScramble {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        let text = text.into();
        Self {
            id: id.into(),
            target_text: text.clone(),
            current_text: text,
            scramble_chars: "!@#$%^&*()_+-=[]{}|;':\",./<>?".into(),
            progress: 1.0,
            font_size: 16.0,
            text_color: rgba(0xffffffff).into(),
            scramble_color: rgba(0x888888ff).into(),
        }
    }

    pub fn current_text(mut self, text: impl Into<SharedString>) -> Self {
        self.current_text = text.into();
        self
    }

    pub fn scramble_chars(mut self, chars: impl Into<SharedString>) -> Self {
        self.scramble_chars = chars.into();
        self
    }

    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
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

    pub fn scramble_color(mut self, color: gpui::Hsla) -> Self {
        self.scramble_color = color;
        self
    }

    /// Check if scramble is complete
    pub fn is_complete(&self) -> bool {
        self.progress >= 1.0
    }
}

impl RenderOnce for TextScramble {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let id = self.id.clone();
        let is_complete = self.is_complete();
        let text_color = self.text_color;
        let scramble_color = self.scramble_color;
        let target_text = self.target_text.clone();
        let current_text = self.current_text.clone();
        let font_size = self.font_size;

        div()
            .id(id)
            .text_size(px(font_size))
            .font_family("monospace")
            .when(is_complete, |d| {
                d.text_color(text_color)
                    .child(target_text)
            })
            .when(!is_complete, |d| {
                d.text_color(scramble_color)
                    .child(current_text)
            })
    }
}
