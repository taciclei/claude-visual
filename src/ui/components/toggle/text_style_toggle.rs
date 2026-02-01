//! Text style toggle component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Bold/Italic/Underline style toggle
#[derive(IntoElement)]
pub struct TextStyleToggle {
    id: ElementId,
    pub(crate) style_type: TextStyleType,
    pressed: bool,
    disabled: bool,
}

impl TextStyleToggle {
    pub fn new(id: impl Into<ElementId>, style_type: TextStyleType) -> Self {
        Self {
            id: id.into(),
            style_type,
            pressed: false,
            disabled: false,
        }
    }

    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = pressed;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub(crate) fn get_icon(&self) -> &'static str {
        match self.style_type {
            TextStyleType::Bold => "𝐁",
            TextStyleType::Italic => "𝐼",
            TextStyleType::Underline => "U̲",
            TextStyleType::Strikethrough => "S̶",
            TextStyleType::Code => "</>",
            TextStyleType::Link => "🔗",
        }
    }
}

impl RenderOnce for TextStyleToggle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let icon = self.get_icon();
        let color = if self.pressed {
            hsla(0.6, 0.7, 0.5, 1.0)
        } else {
            hsla(0.0, 0.0, 0.7, 1.0)
        };

        let mut button = div()
            .id(self.id)
            .size(px(32.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(4.0))
            .cursor_pointer();

        if self.pressed {
            button = button.bg(hsla(0.6, 0.5, 0.5, 0.2));
        } else {
            button = button.hover(|style| style.bg(hsla(0.0, 0.0, 0.15, 1.0)));
        }

        if self.disabled {
            button = button.opacity(0.5).cursor_not_allowed();
        }

        button.child(
            div()
                .text_size(px(14.0))
                .font_weight(if self.style_type == TextStyleType::Bold && self.pressed {
                    gpui::FontWeight::BOLD
                } else {
                    gpui::FontWeight::MEDIUM
                })
                .text_color(color)
                .child(icon),
        )
    }
}
