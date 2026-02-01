//! Emoji button component - single emoji display with click

use gpui::prelude::*;
use gpui::*;

/// Emoji button - single emoji display with click
#[derive(IntoElement)]
pub struct EmojiButton {
    id: ElementId,
    emoji: SharedString,
    size: f32,
    disabled: bool,
}

impl EmojiButton {
    pub fn new(id: impl Into<ElementId>, emoji: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            emoji: emoji.into(),
            size: 24.0,
            disabled: false,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for EmojiButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let opacity = if self.disabled { 0.5 } else { 1.0 };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .w(px(self.size + 8.0))
            .h(px(self.size + 8.0))
            .rounded(px(4.0))
            .opacity(opacity)
            .when(!self.disabled, |el| el.cursor_pointer())
            .child(
                div()
                    .text_size(px(self.size))
                    .child(self.emoji.clone())
            )
    }
}
