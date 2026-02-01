//! Image placeholder component with icon

use gpui::prelude::*;
use gpui::*;

use super::types::ImageShape;

/// Image placeholder with icon
#[derive(IntoElement)]
pub struct ImagePlaceholder {
    width: f32,
    height: f32,
    icon: SharedString,
    label: Option<SharedString>,
    background: Option<Hsla>,
    icon_color: Option<Hsla>,
    shape: ImageShape,
}

impl ImagePlaceholder {
    pub fn new() -> Self {
        Self {
            width: 200.0,
            height: 150.0,
            icon: "🖼".into(),
            label: None,
            background: None,
            icon_color: None,
            shape: ImageShape::Rectangle,
        }
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = icon.into();
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn background(mut self, color: Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn icon_color(mut self, color: Hsla) -> Self {
        self.icon_color = Some(color);
        self
    }

    pub fn shape(mut self, shape: ImageShape) -> Self {
        self.shape = shape;
        self
    }
}

impl Default for ImagePlaceholder {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ImagePlaceholder {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let background = self.background.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.15,
            a: 1.0,
        });
        let icon_color = self.icon_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.4,
            a: 1.0,
        });

        let mut container = div()
            .w(px(self.width))
            .h(px(self.height))
            .bg(background)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_2();

        container = match self.shape {
            ImageShape::Rectangle => container,
            ImageShape::Rounded => container.rounded_lg(),
            ImageShape::Circle => container.rounded_full(),
            ImageShape::Square => container,
        };

        container
            .child(
                div()
                    .text_size(px(32.0))
                    .text_color(icon_color)
                    .child(self.icon),
            )
            .when_some(self.label, |d, label| {
                d.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.5,
                            a: 1.0,
                        })
                        .child(label),
                )
            })
    }
}
