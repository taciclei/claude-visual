//! Simple vertical rule component

use gpui::*;
use gpui::prelude::*;

/// A simple vertical rule (stateless version)
#[derive(Clone)]
pub struct VerticalRule {
    margin: f32,
    color: Option<Hsla>,
}

impl VerticalRule {
    pub fn new() -> Self {
        Self {
            margin: 8.0,
            color: None,
        }
    }

    pub fn with_margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self
    }

    pub fn with_color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl Default for VerticalRule {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for VerticalRule {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or(hsla(0.0, 0.0, 0.5, 0.2));

        div()
            .w(px(1.0))
            .h_full()
            .bg(color)
            .mx(px(self.margin))
    }
}
