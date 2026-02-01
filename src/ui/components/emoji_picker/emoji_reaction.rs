//! Emoji reaction display component - shows reactions on a message

use gpui::prelude::*;
use gpui::*;

/// Emoji reaction display - shows reactions on a message
#[derive(IntoElement)]
pub struct EmojiReaction {
    id: ElementId,
    emoji: SharedString,
    pub(crate) count: u32,
    pub(crate) is_active: bool,
}

impl EmojiReaction {
    pub fn new(id: impl Into<ElementId>, emoji: impl Into<SharedString>, count: u32) -> Self {
        Self {
            id: id.into(),
            emoji: emoji.into(),
            count,
            is_active: false,
        }
    }

    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = is_active;
        self
    }
}

impl RenderOnce for EmojiReaction {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bg = if self.is_active {
            hsla(0.6, 0.5, 0.4, 0.3)
        } else {
            hsla(0.0, 0.0, 0.15, 1.0)
        };

        let border = if self.is_active {
            hsla(0.6, 0.7, 0.5, 1.0)
        } else {
            hsla(0.0, 0.0, 0.25, 1.0)
        };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(4.0))
            .px(px(8.0))
            .py(px(4.0))
            .bg(bg)
            .border_1()
            .border_color(border)
            .rounded(px(12.0))
            .cursor_pointer()
            .child(div().text_size(px(14.0)).child(self.emoji.clone()))
            .child(
                div()
                    .text_size(px(12.0))
                    .font_weight(gpui::FontWeight::MEDIUM)
                    .text_color(if self.is_active {
                        hsla(0.6, 0.7, 0.6, 1.0)
                    } else {
                        hsla(0.0, 0.0, 0.7, 1.0)
                    })
                    .child(self.count.to_string()),
            )
    }
}
