//! Simple horizontal rule component

use gpui::*;
use gpui::prelude::*;

/// A simple horizontal rule (stateless version)
#[derive(Clone)]
pub struct HorizontalRule {
    margin: f32,
    color: Option<Hsla>,
}

impl HorizontalRule {
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

impl Default for HorizontalRule {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for HorizontalRule {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or(hsla(0.0, 0.0, 0.5, 0.2));

        div()
            .w_full()
            .h(px(1.0))
            .bg(color)
            .my(px(self.margin))
    }
}
