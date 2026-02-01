//! Simple color swatch for display

use gpui::prelude::*;
use gpui::*;

#[derive(Clone)]
pub struct ColorSwatch {
    color: Hsla,
    size: f32,
    rounded: bool,
    show_border: bool,
}

impl ColorSwatch {
    pub fn new(color: Hsla) -> Self {
        Self {
            color,
            size: 24.0,
            rounded: false,
            show_border: true,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn rounded(mut self) -> Self {
        self.rounded = true;
        self
    }

    pub fn no_border(mut self) -> Self {
        self.show_border = false;
        self
    }
}

impl RenderOnce for ColorSwatch {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        div()
            .size(px(self.size))
            .when(self.rounded, |d| d.rounded_full())
            .when(!self.rounded, |d| d.rounded(px(4.0)))
            .when(self.show_border, |d| d.border_1().border_color(border))
            .bg(self.color)
    }
}
