//! Thumbnail image component with optional badge

use gpui::prelude::*;
use gpui::*;

use super::image::Image;
use super::types::{ImageShape, ImageState};

/// Thumbnail image with optional badge
#[derive(IntoElement)]
pub struct Thumbnail {
    src: SharedString,
    size: f32,
    shape: ImageShape,
    badge: Option<SharedString>,
    badge_color: Option<Hsla>,
    border: bool,
}

impl Thumbnail {
    pub fn new(src: impl Into<SharedString>) -> Self {
        Self {
            src: src.into(),
            size: 48.0,
            shape: ImageShape::Square,
            badge: None,
            badge_color: None,
            border: false,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn shape(mut self, shape: ImageShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn badge(mut self, badge: impl Into<SharedString>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn badge_color(mut self, color: Hsla) -> Self {
        self.badge_color = Some(color);
        self
    }

    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }
}

impl RenderOnce for Thumbnail {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let badge_color = self.badge_color.unwrap_or(Hsla {
            h: 0.58,
            s: 0.7,
            l: 0.5,
            a: 1.0,
        });

        let image = Image::new(self.src)
            .size(self.size)
            .shape(self.shape)
            .state(ImageState::Loaded)
            .border(self.border);

        div()
            .relative()
            .child(image)
            .when_some(self.badge, |d, badge| {
                d.child(
                    div()
                        .absolute()
                        .top(px(-4.0))
                        .right(px(-4.0))
                        .bg(badge_color)
                        .px(px(6.0))
                        .py(px(2.0))
                        .rounded_full()
                        .text_size(px(10.0))
                        .text_color(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 1.0,
                            a: 1.0,
                        })
                        .child(badge),
                )
            })
    }
}
