//! Bookmark toggle component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Bookmark toggle
#[derive(IntoElement)]
pub struct BookmarkToggle {
    id: ElementId,
    bookmarked: bool,
    size: ToggleSize,
    disabled: bool,
}

impl BookmarkToggle {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            bookmarked: false,
            size: ToggleSize::default(),
            disabled: false,
        }
    }

    pub fn bookmarked(mut self, bookmarked: bool) -> Self {
        self.bookmarked = bookmarked;
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
}

impl RenderOnce for BookmarkToggle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let icon_size = match self.size {
            ToggleSize::Small => 14.0,
            ToggleSize::Medium => 18.0,
            ToggleSize::Large => 22.0,
        };

        let (icon, color) = if self.bookmarked {
            ("🔖", hsla(0.6, 0.7, 0.5, 1.0))
        } else {
            ("🏷️", hsla(0.0, 0.0, 0.5, 1.0))
        };

        let mut button = div()
            .id(self.id)
            .size(px(icon_size * 2.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(4.0))
            .cursor_pointer();

        if !self.disabled {
            button = button.hover(|style| style.bg(hsla(0.0, 0.0, 0.15, 1.0)));
        } else {
            button = button.opacity(0.5).cursor_not_allowed();
        }

        button.child(div().text_size(px(icon_size)).text_color(color).child(icon))
    }
}
