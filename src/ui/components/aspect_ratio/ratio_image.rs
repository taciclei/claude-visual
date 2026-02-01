use super::types::{ImageFit, Ratio};
use gpui::prelude::*;
use gpui::*;

/// Image with aspect ratio preservation
#[derive(IntoElement)]
pub struct RatioImage {
    src: SharedString,
    alt: SharedString,
    ratio: Ratio,
    width: Option<f32>,
    fit: ImageFit,
    border: bool,
    rounded: bool,
}

impl RatioImage {
    pub fn new(src: impl Into<SharedString>) -> Self {
        Self {
            src: src.into(),
            alt: "".into(),
            ratio: Ratio::Square,
            width: None,
            fit: ImageFit::Cover,
            border: false,
            rounded: false,
        }
    }

    pub fn alt(mut self, alt: impl Into<SharedString>) -> Self {
        self.alt = alt.into();
        self
    }

    pub fn ratio(mut self, ratio: Ratio) -> Self {
        self.ratio = ratio;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn fit(mut self, fit: ImageFit) -> Self {
        self.fit = fit;
        self
    }

    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }

    pub fn rounded(mut self, rounded: bool) -> Self {
        self.rounded = rounded;
        self
    }
}

impl RenderOnce for RatioImage {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let width = self.width.unwrap_or(200.0);
        let height = width / self.ratio.value();

        let mut container = div()
            .relative()
            .w(px(width))
            .h(px(height))
            .bg(Hsla {
                h: 0.0,
                s: 0.0,
                l: 0.15,
                a: 1.0,
            })
            .overflow_hidden();

        if self.border {
            container = container.border_1().border_color(Hsla {
                h: 0.0,
                s: 0.0,
                l: 0.3,
                a: 1.0,
            });
        }

        if self.rounded {
            container = container.rounded_lg();
        }

        // Placeholder showing src path
        container.child(
            div()
                .absolute()
                .inset_0()
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(10.0))
                .text_color(Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.5,
                    a: 1.0,
                })
                .p_2()
                .child(self.src),
        )
    }
}
