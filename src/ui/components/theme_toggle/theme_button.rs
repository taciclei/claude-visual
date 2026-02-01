//! Simple icon-only theme button

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Simple icon-only theme button
#[derive(IntoElement)]
pub struct ThemeButton {
    id: ElementId,
    mode: ThemeMode,
    size: f32,
}

impl ThemeButton {
    pub fn new(id: impl Into<ElementId>, mode: ThemeMode) -> Self {
        Self {
            id: id.into(),
            mode,
            size: 32.0,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl RenderOnce for ThemeButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .w(px(self.size))
            .h(px(self.size))
            .rounded(px(6.0))
            .cursor_pointer()
            .child(
                div()
                    .text_size(px(self.size * 0.55))
                    .child(self.mode.icon()),
            )
    }
}
