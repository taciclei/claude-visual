//! Simple star rating display component

use gpui::*;
use gpui::prelude::*;

use super::types::*;

/// Simple star rating display
#[derive(Clone)]
pub struct StarRating {
    pub(crate) value: f32,
    pub(crate) max: u8,
    pub(crate) size: RatingSize,
}

impl StarRating {
    pub fn new(value: f32, max: u8) -> Self {
        Self {
            value: value.clamp(0.0, max as f32),
            max,
            size: RatingSize::default(),
        }
    }

    pub fn size(mut self, size: RatingSize) -> Self {
        self.size = size;
        self
    }
}

impl RenderOnce for StarRating {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let icon_size = self.size.icon_size();
        let filled_color = hsla(0.12, 0.9, 0.5, 1.0); // Gold
        let empty_color = hsla(0.0, 0.0, 0.4, 1.0);

        div()
            .flex()
            .items_center()
            .gap(px(2.0))
            .children((0..self.max).map(|i| {
                let filled = self.value >= (i as f32 + 1.0);
                let half = !filled && self.value > i as f32;

                let icon = if filled { "★" } else if half { "⯪" } else { "☆" };
                let color = if filled || half { filled_color } else { empty_color };

                div()
                    .text_size(px(icon_size))
                    .text_color(color)
                    .child(icon)
            }))
    }
}
