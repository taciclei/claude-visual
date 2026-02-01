//! Simple stateless card wrapper

use gpui::prelude::*;
use gpui::*;

/// Simple stateless card wrapper
#[derive(Clone)]
pub struct SimpleCard {
    pub(crate) padding: f32,
    pub(crate) rounded: f32,
    pub(crate) bordered: bool,
}

impl SimpleCard {
    pub fn new() -> Self {
        Self {
            padding: 16.0,
            rounded: 8.0,
            bordered: true,
        }
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_rounded(mut self, rounded: f32) -> Self {
        self.rounded = rounded;
        self
    }

    pub fn without_border(mut self) -> Self {
        self.bordered = false;
        self
    }
}

impl Default for SimpleCard {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for SimpleCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let bg = hsla(0.0, 0.0, 0.15, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        div()
            .p(px(self.padding))
            .rounded(px(self.rounded))
            .bg(bg)
            .when(self.bordered, |d| d.border_1().border_color(border))
    }
}
