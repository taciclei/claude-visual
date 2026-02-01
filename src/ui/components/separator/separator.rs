//! Basic separator component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// A basic separator component
#[derive(IntoElement)]
pub struct Separator {
    orientation: SeparatorOrientation,
    style: SeparatorStyle,
    thickness: SeparatorThickness,
    length: Option<f32>,
    color: Option<Hsla>,
    margin: f32,
}

impl Separator {
    pub fn new() -> Self {
        Self {
            orientation: SeparatorOrientation::Horizontal,
            style: SeparatorStyle::Solid,
            thickness: SeparatorThickness::Default,
            length: None,
            color: None,
            margin: 8.0,
        }
    }

    pub fn horizontal() -> Self {
        Self::new().orientation(SeparatorOrientation::Horizontal)
    }

    pub fn vertical() -> Self {
        Self::new().orientation(SeparatorOrientation::Vertical)
    }

    pub fn orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn style(mut self, style: SeparatorStyle) -> Self {
        self.style = style;
        self
    }

    pub fn thickness(mut self, thickness: SeparatorThickness) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn length(mut self, length: f32) -> Self {
        self.length = Some(length);
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self
    }
}

impl Default for Separator {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Separator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.25,
            a: 1.0,
        });
        let thickness = self.thickness.pixels();

        match self.orientation {
            SeparatorOrientation::Horizontal => {
                let mut sep = div().h(px(thickness)).my(px(self.margin)).bg(color);

                if let Some(length) = self.length {
                    sep = sep.w(px(length));
                } else {
                    sep = sep.w_full();
                }

                sep
            }
            SeparatorOrientation::Vertical => {
                let mut sep = div().w(px(thickness)).mx(px(self.margin)).bg(color);

                if let Some(length) = self.length {
                    sep = sep.h(px(length));
                } else {
                    sep = sep.h_full();
                }

                sep
            }
        }
    }
}
