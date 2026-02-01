//! Stateless skeleton line component

use gpui::prelude::*;
use gpui::*;

/// Stateless skeleton for simple use cases
#[derive(Clone)]
pub struct SkeletonLine {
    pub(crate) height: f32,
    pub(crate) width: Option<f32>,
    rounded: bool,
}

impl SkeletonLine {
    pub fn new() -> Self {
        Self {
            height: 12.0,
            width: None,
            rounded: true,
        }
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn full_width(mut self) -> Self {
        self.width = None;
        self
    }
}

impl Default for SkeletonLine {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for SkeletonLine {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = hsla(0.0, 0.0, 0.5, 0.15);
        let radius = if self.rounded { self.height / 2.0 } else { 2.0 };

        div()
            .h(px(self.height))
            .when_some(self.width, |d, w| d.w(px(w)))
            .when(self.width.is_none(), |d| d.w_full())
            .rounded(px(radius))
            .bg(color)
    }
}
