//! Toolbar group component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Toolbar group - groups related toolbar items
#[derive(IntoElement)]
pub struct ToolbarGroup {
    items: Vec<ToolbarItem>,
    size: ToolbarSize,
    vertical: bool,
    background: Option<gpui::Hsla>,
    border_color: Option<gpui::Hsla>,
}

impl ToolbarGroup {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            size: ToolbarSize::default(),
            vertical: false,
            background: None,
            border_color: None,
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

    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn border_color(mut self, color: gpui::Hsla) -> Self {
        self.border_color = Some(color);
        self
    }
}

impl Default for ToolbarGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ToolbarGroup {
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

        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.12, 1.0));
        let border = self.border_color.unwrap_or(hsla(0.0, 0.0, 0.2, 1.0));

        let mut group = div()
            .rounded(px(6.0))
            .border_1()
            .border_color(border)
            .bg(bg)
            .overflow_hidden();

        group = if self.vertical {
            group.flex().flex_col()
        } else {
            group.flex()
        };

        group.children(self.items.into_iter().enumerate().map(|(i, item)| {
            let mut button = div()
                .size(px(button_size))
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer();

            // Add separator between items
            if i > 0 {
                button = if self.vertical {
                    button.border_t_1().border_color(border)
                } else {
                    button.border_l_1().border_color(border)
                };
            }

            if item.active {
                button = button.bg(hsla(0.6, 0.5, 0.4, 0.3));
            }

            if item.disabled {
                button = button.opacity(0.5).cursor_not_allowed();
            } else if !item.active {
                button = button.hover(|style| style.bg(hsla(0.0, 0.0, 0.2, 1.0)));
            }

            button.child(
                div()
                    .text_size(px(icon_size))
                    .text_color(if item.active {
                        hsla(0.6, 0.7, 0.6, 1.0)
                    } else {
                        hsla(0.0, 0.0, 0.7, 1.0)
                    })
                    .child(item.icon.clone()),
            )
        }))
    }
}
