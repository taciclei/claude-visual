//! Color palette display

use gpui::*;
use gpui::prelude::*;

#[derive(Clone)]
pub struct ColorPalette {
    pub(crate) colors: Vec<(String, Hsla)>,
    pub(crate) swatch_size: f32,
    pub(crate) columns: usize,
}

impl ColorPalette {
    pub fn new() -> Self {
        Self {
            colors: Vec::new(),
            swatch_size: 32.0,
            columns: 5,
        }
    }

    pub fn color(mut self, name: impl Into<String>, color: Hsla) -> Self {
        self.colors.push((name.into(), color));
        self
    }

    pub fn swatch_size(mut self, size: f32) -> Self {
        self.swatch_size = size;
        self
    }

    pub fn columns(mut self, cols: usize) -> Self {
        self.columns = cols.max(1);
        self
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ColorPalette {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let hover = hsla(0.0, 0.0, 0.18, 1.0);

        let width = (self.swatch_size + 8.0) * self.columns as f32;

        div()
            .w(px(width))
            .flex()
            .flex_wrap()
            .gap_2()
            .children(self.colors.into_iter().map(|(name, color)| {
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_1()
                    .p_1()
                    .rounded(px(4.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(hover))
                    .child(
                        div()
                            .size(px(self.swatch_size))
                            .rounded(px(4.0))
                            .border_1()
                            .border_color(border)
                            .bg(color)
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(text_muted)
                            .max_w(px(self.swatch_size + 16.0))
                            .text_ellipsis()
                            .overflow_hidden()
                            .child(name)
                    )
            }))
    }
}
