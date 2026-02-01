//! Lazy-loaded image component

use gpui::prelude::*;
use gpui::*;

use super::image::Image;
use super::types::ImageState;

/// Lazy-loaded image
#[derive(IntoElement)]
pub struct LazyImage {
    src: SharedString,
    placeholder: Div,
    width: Option<f32>,
    height: Option<f32>,
    loaded: bool,
}

impl LazyImage {
    pub fn new(src: impl Into<SharedString>) -> Self {
        Self {
            src: src.into(),
            placeholder: div(),
            width: None,
            height: None,
            loaded: false,
        }
    }

    pub fn placeholder(mut self, placeholder: impl IntoElement) -> Self {
        self.placeholder = div().child(placeholder);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn loaded(mut self, loaded: bool) -> Self {
        self.loaded = loaded;
        self
    }
}

impl RenderOnce for LazyImage {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut container = div();

        if let Some(w) = self.width {
            container = container.w(px(w));
        }
        if let Some(h) = self.height {
            container = container.h(px(h));
        }

        if self.loaded {
            container.child(
                Image::new(self.src)
                    .width(self.width.unwrap_or(100.0))
                    .height(self.height.unwrap_or(100.0))
                    .state(ImageState::Loaded),
            )
        } else {
            container.child(self.placeholder)
        }
    }
}
