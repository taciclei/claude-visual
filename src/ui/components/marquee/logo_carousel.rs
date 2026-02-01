//! Logo carousel - common use case for marquees

use gpui::prelude::*;
use gpui::*;

use super::types::*;

/// Logo carousel - common use case for marquees
#[derive(IntoElement)]
pub struct LogoCarousel {
    pub(crate) id: ElementId,
    pub(crate) logos: Vec<SharedString>,
    pub(crate) direction: MarqueeDirection,
    pub(crate) speed: MarqueeSpeed,
    pub(crate) logo_size: f32,
    pub(crate) grayscale: bool,
}

impl LogoCarousel {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            logos: Vec::new(),
            direction: MarqueeDirection::default(),
            speed: MarqueeSpeed::default(),
            logo_size: 48.0,
            grayscale: true,
        }
    }

    pub fn logos(mut self, logos: Vec<impl Into<SharedString>>) -> Self {
        self.logos = logos.into_iter().map(|l| l.into()).collect();
        self
    }

    pub fn direction(mut self, direction: MarqueeDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn speed(mut self, speed: MarqueeSpeed) -> Self {
        self.speed = speed;
        self
    }

    pub fn logo_size(mut self, size: f32) -> Self {
        self.logo_size = size;
        self
    }

    pub fn grayscale(mut self, grayscale: bool) -> Self {
        self.grayscale = grayscale;
        self
    }
}

impl RenderOnce for LogoCarousel {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .w_full()
            .overflow_hidden()
            .py(px(16.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(48.0))
                    .children(self.logos.iter().map(|logo| {
                        div()
                            .h(px(self.logo_size))
                            .flex()
                            .items_center()
                            .justify_center()
                            .when(self.grayscale, |el| el.opacity(0.6))
                            .child(
                                div()
                                    .text_size(px(self.logo_size * 0.6))
                                    .child(logo.clone()),
                            )
                    })),
            )
    }
}
