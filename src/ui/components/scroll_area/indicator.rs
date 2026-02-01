//! Scroll position indicator component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Scroll indicator showing scroll position
#[derive(IntoElement)]
pub struct ScrollIndicator {
    position: f32, // 0.0 to 1.0
    direction: ScrollDirection,
    size: ScrollbarSize,
    thumb_color: Option<Hsla>,
    track_color: Option<Hsla>,
    length: f32,
    visible_ratio: f32, // How much of the content is visible (0.0 to 1.0)
}

impl ScrollIndicator {
    pub fn new() -> Self {
        Self {
            position: 0.0,
            direction: ScrollDirection::Vertical,
            size: ScrollbarSize::Default,
            thumb_color: None,
            track_color: None,
            length: 200.0,
            visible_ratio: 0.3,
        }
    }

    pub fn position(mut self, position: f32) -> Self {
        self.position = position.clamp(0.0, 1.0);
        self
    }

    pub fn direction(mut self, direction: ScrollDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn size(mut self, size: ScrollbarSize) -> Self {
        self.size = size;
        self
    }

    pub fn thumb_color(mut self, color: Hsla) -> Self {
        self.thumb_color = Some(color);
        self
    }

    pub fn track_color(mut self, color: Hsla) -> Self {
        self.track_color = Some(color);
        self
    }

    pub fn length(mut self, length: f32) -> Self {
        self.length = length;
        self
    }

    pub fn visible_ratio(mut self, ratio: f32) -> Self {
        self.visible_ratio = ratio.clamp(0.0, 1.0);
        self
    }
}

impl Default for ScrollIndicator {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ScrollIndicator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let thumb_color = self.thumb_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.5,
            a: 0.7,
        });
        let track_color = self.track_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.2,
            a: 0.5,
        });

        let bar_width = self.size.width();
        let thumb_length = self.length * self.visible_ratio;
        let max_offset = self.length - thumb_length;
        let thumb_offset = max_offset * self.position;

        match self.direction {
            ScrollDirection::Vertical | ScrollDirection::Both => div()
                .w(px(bar_width))
                .h(px(self.length))
                .bg(track_color)
                .rounded_full()
                .relative()
                .child(
                    div()
                        .absolute()
                        .top(px(thumb_offset))
                        .left_0()
                        .w(px(bar_width))
                        .h(px(thumb_length))
                        .bg(thumb_color)
                        .rounded_full(),
                ),
            ScrollDirection::Horizontal => div()
                .h(px(bar_width))
                .w(px(self.length))
                .bg(track_color)
                .rounded_full()
                .relative()
                .child(
                    div()
                        .absolute()
                        .left(px(thumb_offset))
                        .top_0()
                        .h(px(bar_width))
                        .w(px(thumb_length))
                        .bg(thumb_color)
                        .rounded_full(),
                ),
        }
    }
}
