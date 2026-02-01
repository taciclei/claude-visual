//! Gap component with optional line

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Gap component with optional line
#[derive(IntoElement)]
pub struct Gap {
    size: f32,
    show_line: bool,
    line_color: Option<Hsla>,
    orientation: SeparatorOrientation,
}

impl Gap {
    pub fn new(size: f32) -> Self {
        Self {
            size,
            show_line: false,
            line_color: None,
            orientation: SeparatorOrientation::Horizontal,
        }
    }

    pub fn show_line(mut self, show: bool) -> Self {
        self.show_line = show;
        self
    }

    pub fn line_color(mut self, color: Hsla) -> Self {
        self.line_color = Some(color);
        self
    }

    pub fn vertical() -> Self {
        Self {
            size: 16.0,
            show_line: false,
            line_color: None,
            orientation: SeparatorOrientation::Vertical,
        }
    }
}

impl RenderOnce for Gap {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let line_color = self.line_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.25,
            a: 1.0,
        });

        match self.orientation {
            SeparatorOrientation::Horizontal => {
                let mut gap = div().h(px(self.size)).w_full();
                if self.show_line {
                    gap = gap.flex().items_center().child(
                        div().h(px(1.0)).w_full().bg(line_color),
                    );
                }
                gap
            }
            SeparatorOrientation::Vertical => {
                let mut gap = div().w(px(self.size)).h_full();
                if self.show_line {
                    gap = gap.flex().items_center().child(
                        div().w(px(1.0)).h_full().bg(line_color),
                    );
                }
                gap
            }
        }
    }
}
