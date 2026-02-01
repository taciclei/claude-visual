//! Toolbar separator component

use gpui::prelude::*;
use gpui::*;

/// Toolbar separator - visual divider between toolbar items
#[derive(IntoElement)]
pub struct ToolbarSeparator {
    vertical: bool,
    color: Option<gpui::Hsla>,
}

impl ToolbarSeparator {
    pub fn new() -> Self {
        Self {
            vertical: true,
            color: None,
        }
    }

    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    pub fn color(mut self, color: gpui::Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl Default for ToolbarSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ToolbarSeparator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or(hsla(0.0, 0.0, 0.25, 1.0));

        if self.vertical {
            div().w(px(1.0)).h(px(24.0)).mx(px(4.0)).bg(color)
        } else {
            div().h(px(1.0)).w(px(24.0)).my(px(4.0)).bg(color)
        }
    }
}
