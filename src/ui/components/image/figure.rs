//! Image with caption component

use gpui::prelude::*;
use gpui::*;

use super::image::Image;
use super::types::CaptionPosition;

/// Image with caption
#[derive(IntoElement)]
pub struct Figure {
    image: Image,
    caption: Option<SharedString>,
    caption_position: CaptionPosition,
    caption_color: Option<Hsla>,
}

impl Figure {
    pub fn new(image: Image) -> Self {
        Self {
            image,
            caption: None,
            caption_position: CaptionPosition::Bottom,
            caption_color: None,
        }
    }

    pub fn caption(mut self, caption: impl Into<SharedString>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    pub fn caption_position(mut self, position: CaptionPosition) -> Self {
        self.caption_position = position;
        self
    }

    pub fn caption_color(mut self, color: Hsla) -> Self {
        self.caption_color = Some(color);
        self
    }
}

impl RenderOnce for Figure {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let caption_color = self.caption_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.6,
            a: 1.0,
        });

        let caption_el = self.caption.as_ref().map(|text| {
            div()
                .text_size(px(12.0))
                .text_color(caption_color)
                .py_1()
                .child(text.clone())
        });

        match self.caption_position {
            CaptionPosition::Top => div()
                .flex()
                .flex_col()
                .when_some(caption_el, |d, c| d.child(c))
                .child(self.image),
            CaptionPosition::Bottom => div()
                .flex()
                .flex_col()
                .child(self.image)
                .when_some(caption_el, |d, c| d.child(c)),
            CaptionPosition::Overlay => {
                div()
                    .relative()
                    .child(self.image)
                    .when_some(self.caption.clone(), |d, text| {
                        d.child(
                            div()
                                .absolute()
                                .bottom_0()
                                .left_0()
                                .right_0()
                                .bg(Hsla {
                                    h: 0.0,
                                    s: 0.0,
                                    l: 0.0,
                                    a: 0.7,
                                })
                                .px_2()
                                .py_1()
                                .text_size(px(12.0))
                                .text_color(Hsla {
                                    h: 0.0,
                                    s: 0.0,
                                    l: 1.0,
                                    a: 1.0,
                                })
                                .child(text),
                        )
                    })
            }
        }
    }
}
