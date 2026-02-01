//! Floating action toolbar component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Floating action toolbar - contextual actions
#[derive(IntoElement)]
pub struct FloatingToolbar {
    id: ElementId,
    items: Vec<ToolbarItem>,
    size: ToolbarSize,
    position: (f32, f32),
    background: Option<gpui::Hsla>,
}

impl FloatingToolbar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            size: ToolbarSize::default(),
            position: (0.0, 0.0),
            background: None,
        }
    }

    pub fn items(mut self, items: Vec<ToolbarItem>) -> Self {
        self.items = items;
        self
    }

    pub fn size(mut self, size: ToolbarSize) -> Self {
        self.size = size;
        self
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = (x, y);
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = Some(color);
        self
    }
}

impl RenderOnce for FloatingToolbar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let button_size = match self.size {
            ToolbarSize::Small => 28.0,
            ToolbarSize::Medium => 36.0,
            ToolbarSize::Large => 44.0,
        };
        let icon_size = match self.size {
            ToolbarSize::Small => 12.0,
            ToolbarSize::Medium => 14.0,
            ToolbarSize::Large => 16.0,
        };

        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.1, 0.95));

        div()
            .id(self.id)
            .absolute()
            .left(px(self.position.0))
            .top(px(self.position.1))
            .flex()
            .items_center()
            .gap(px(2.0))
            .px(px(4.0))
            .py(px(4.0))
            .rounded(px(8.0))
            .bg(bg)
            .shadow_xl()
            .children(self.items.into_iter().map(|item| {
                let mut button = div()
                    .size(px(button_size))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(6.0))
                    .cursor_pointer();

                if item.active {
                    button = button.bg(hsla(0.6, 0.5, 0.4, 0.3));
                }

                if item.disabled {
                    button = button.opacity(0.5).cursor_not_allowed();
                } else {
                    button = button.hover(|style| style.bg(hsla(0.0, 0.0, 0.2, 1.0)));
                }

                button.child(
                    div()
                        .text_size(px(icon_size))
                        .text_color(if item.active {
                            hsla(0.6, 0.7, 0.6, 1.0)
                        } else {
                            hsla(0.0, 0.0, 0.8, 1.0)
                        })
                        .child(item.icon.clone()),
                )
            }))
    }
}
