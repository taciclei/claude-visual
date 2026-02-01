//! Standalone resize handle component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Resize handle component (standalone)
#[derive(IntoElement)]
pub struct ResizeHandle {
    direction: ResizeDirection,
    style: HandleStyle,
    pub(crate) size: f32,
    color: Option<Hsla>,
    hover_color: Option<Hsla>,
    pub(crate) active: bool,
}

impl ResizeHandle {
    pub fn new() -> Self {
        Self {
            direction: ResizeDirection::Horizontal,
            style: HandleStyle::Line,
            size: 8.0,
            color: None,
            hover_color: None,
            active: false,
        }
    }

    pub fn horizontal() -> Self {
        Self::new().direction(ResizeDirection::Horizontal)
    }

    pub fn vertical() -> Self {
        Self::new().direction(ResizeDirection::Vertical)
    }

    pub fn direction(mut self, direction: ResizeDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn style(mut self, style: HandleStyle) -> Self {
        self.style = style;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn hover_color(mut self, color: Hsla) -> Self {
        self.hover_color = Some(color);
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

impl Default for ResizeHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ResizeHandle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.3,
            a: 1.0,
        });
        let hover_color = self.hover_color.unwrap_or(Hsla {
            h: 0.58,
            s: 0.7,
            l: 0.5,
            a: 1.0,
        });

        let display_color = if self.active { hover_color } else { color };
        let is_horizontal = matches!(self.direction, ResizeDirection::Horizontal);

        let mut handle = div().flex_shrink_0().flex().items_center().justify_center();

        if is_horizontal {
            handle = handle.w(px(self.size)).h_full().cursor_ew_resize();
        } else {
            handle = handle.h(px(self.size)).w_full().cursor_ns_resize();
        }

        let indicator = match self.style {
            HandleStyle::Line => {
                if is_horizontal {
                    div()
                        .w(px(2.0))
                        .h(px(24.0))
                        .bg(display_color)
                        .rounded_full()
                } else {
                    div()
                        .h(px(2.0))
                        .w(px(24.0))
                        .bg(display_color)
                        .rounded_full()
                }
            }
            HandleStyle::Dots => div()
                .flex()
                .when(is_horizontal, |d| d.flex_col())
                .gap(px(3.0))
                .child(div().w(px(4.0)).h(px(4.0)).rounded_full().bg(display_color))
                .child(div().w(px(4.0)).h(px(4.0)).rounded_full().bg(display_color))
                .child(div().w(px(4.0)).h(px(4.0)).rounded_full().bg(display_color)),
            HandleStyle::Grip => div()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(
                    div()
                        .w(px(12.0))
                        .h(px(2.0))
                        .bg(display_color)
                        .rounded_full(),
                )
                .child(
                    div()
                        .w(px(12.0))
                        .h(px(2.0))
                        .bg(display_color)
                        .rounded_full(),
                ),
            HandleStyle::Hidden => div(),
        };

        handle
            .hover(|s| s.bg(hover_color.opacity(0.5)))
            .child(indicator)
    }
}
