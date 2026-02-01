//! Main image component with placeholder and error states

use gpui::*;
use gpui::prelude::*;

use super::types::{ImageFit, ImageState, ImageShape};

/// A basic image component with placeholder and error states
#[derive(IntoElement)]
pub struct Image {
    pub(crate) src: SharedString,
    pub(crate) alt: SharedString,
    pub(crate) width: Option<f32>,
    pub(crate) height: Option<f32>,
    pub(crate) fit: ImageFit,
    pub(crate) shape: ImageShape,
    pub(crate) state: ImageState,
    pub(crate) placeholder_color: Option<Hsla>,
    pub(crate) border: bool,
    pub(crate) border_color: Option<Hsla>,
    pub(crate) shadow: bool,
}

impl Image {
    pub fn new(src: impl Into<SharedString>) -> Self {
        Self {
            src: src.into(),
            alt: "".into(),
            width: None,
            height: None,
            fit: ImageFit::Cover,
            shape: ImageShape::Rectangle,
            state: ImageState::Loading,
            placeholder_color: None,
            border: false,
            border_color: None,
            shadow: false,
        }
    }

    pub fn alt(mut self, alt: impl Into<SharedString>) -> Self {
        self.alt = alt.into();
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

    pub fn size(mut self, size: f32) -> Self {
        self.width = Some(size);
        self.height = Some(size);
        self
    }

    pub fn fit(mut self, fit: ImageFit) -> Self {
        self.fit = fit;
        self
    }

    pub fn shape(mut self, shape: ImageShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn state(mut self, state: ImageState) -> Self {
        self.state = state;
        self
    }

    pub fn placeholder_color(mut self, color: Hsla) -> Self {
        self.placeholder_color = Some(color);
        self
    }

    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }

    pub fn border_color(mut self, color: Hsla) -> Self {
        self.border_color = Some(color);
        self
    }

    pub fn shadow(mut self, shadow: bool) -> Self {
        self.shadow = shadow;
        self
    }
}

impl RenderOnce for Image {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let placeholder_color = self.placeholder_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.2,
            a: 1.0,
        });
        let border_color = self.border_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.3,
            a: 1.0,
        });

        let mut container = div().relative().overflow_hidden();

        // Apply dimensions
        if let Some(w) = self.width {
            container = container.w(px(w));
        }
        if let Some(h) = self.height {
            container = container.h(px(h));
        }

        // Apply shape
        container = match self.shape {
            ImageShape::Rectangle => container,
            ImageShape::Rounded => container.rounded_lg(),
            ImageShape::Circle => container.rounded_full(),
            ImageShape::Square => {
                let size = self.width.or(self.height).unwrap_or(100.0);
                container.w(px(size)).h(px(size))
            }
        };

        // Apply border
        if self.border {
            container = container.border_1().border_color(border_color);
        }

        // Apply shadow
        if self.shadow {
            container = container.shadow_md();
        }

        // Render based on state
        match self.state {
            ImageState::Loading => {
                container = container.bg(placeholder_color).child(
                    div()
                        .absolute()
                        .inset_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(Hsla {
                                    h: 0.0,
                                    s: 0.0,
                                    l: 0.5,
                                    a: 1.0,
                                })
                                .child("Loading..."),
                        ),
                );
            }
            ImageState::Loaded => {
                // In a real implementation, this would render the actual image
                // For now, show a placeholder with the src path
                container = container.bg(placeholder_color).child(
                    div()
                        .absolute()
                        .inset_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(11.0))
                        .text_color(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.6,
                            a: 1.0,
                        })
                        .child(self.src),
                );
            }
            ImageState::Error => {
                container = container.bg(placeholder_color).child(
                    div()
                        .absolute()
                        .inset_0()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .gap_1()
                        .child(
                            div()
                                .text_size(px(20.0))
                                .text_color(Hsla {
                                    h: 0.0,
                                    s: 0.6,
                                    l: 0.5,
                                    a: 1.0,
                                })
                                .child("✕"),
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(Hsla {
                                    h: 0.0,
                                    s: 0.0,
                                    l: 0.5,
                                    a: 1.0,
                                })
                                .child("Failed to load"),
                        ),
                );
            }
        }

        container
    }
}
