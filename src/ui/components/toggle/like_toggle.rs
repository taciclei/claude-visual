//! Like/Heart toggle component

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Like/Heart toggle
#[derive(IntoElement)]
pub struct LikeToggle {
    id: ElementId,
    pub(crate) liked: bool,
    size: ToggleSize,
    disabled: bool,
    pub(crate) count: Option<u32>,
    animate: bool,
}

impl LikeToggle {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            liked: false,
            size: ToggleSize::default(),
            disabled: false,
            count: None,
            animate: true,
        }
    }

    pub fn liked(mut self, liked: bool) -> Self {
        self.liked = liked;
        self
    }

    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }

    pub fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }
}

impl RenderOnce for LikeToggle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let icon_size = match self.size {
            ToggleSize::Small => 14.0,
            ToggleSize::Medium => 18.0,
            ToggleSize::Large => 22.0,
        };

        let (icon, color) = if self.liked {
            ("❤️", hsla(0.0, 0.8, 0.5, 1.0))
        } else {
            ("🤍", hsla(0.0, 0.0, 0.5, 1.0))
        };

        let mut button = div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(4.0))
            .cursor_pointer();

        if self.disabled {
            button = button.opacity(0.5).cursor_not_allowed();
        }

        button
            .child(
                div()
                    .text_size(px(icon_size))
                    .text_color(color)
                    .child(icon)
            )
            .when(self.count.is_some(), |el| {
                el.child(
                    div()
                        .text_size(px(icon_size * 0.7))
                        .text_color(hsla(0.0, 0.0, 0.6, 1.0))
                        .child(self.count.unwrap().to_string())
                )
            })
    }
}
