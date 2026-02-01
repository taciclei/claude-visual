//! Spacer component for flex layouts

use gpui::*;
use gpui::prelude::*;

/// Spacer component for flex layouts
#[derive(IntoElement)]
pub struct Spacer {
    size: Option<f32>,
    flex: bool,
}

impl Spacer {
    pub fn new() -> Self {
        Self {
            size: None,
            flex: true,
        }
    }

    pub fn fixed(size: f32) -> Self {
        Self {
            size: Some(size),
            flex: false,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self.flex = false;
        self
    }
}

impl Default for Spacer {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Spacer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        if self.flex {
            div().flex_1()
        } else if let Some(size) = self.size {
            div().w(px(size)).h(px(size))
        } else {
            div()
        }
    }
}
