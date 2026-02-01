//! Scroll navigation buttons

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Scroll to top/bottom buttons
#[derive(IntoElement)]
pub struct ScrollButtons {
    show_top: bool,
    show_bottom: bool,
    button_size: f32,
    button_color: Option<Hsla>,
    icon_color: Option<Hsla>,
    position: ScrollButtonPosition,
}

impl ScrollButtons {
    pub fn new() -> Self {
        Self {
            show_top: true,
            show_bottom: true,
            button_size: 32.0,
            button_color: None,
            icon_color: None,
            position: ScrollButtonPosition::Right,
        }
    }

    pub fn show_top(mut self, show: bool) -> Self {
        self.show_top = show;
        self
    }

    pub fn show_bottom(mut self, show: bool) -> Self {
        self.show_bottom = show;
        self
    }

    pub fn button_size(mut self, size: f32) -> Self {
        self.button_size = size;
        self
    }

    pub fn button_color(mut self, color: Hsla) -> Self {
        self.button_color = Some(color);
        self
    }

    pub fn icon_color(mut self, color: Hsla) -> Self {
        self.icon_color = Some(color);
        self
    }

    pub fn position(mut self, position: ScrollButtonPosition) -> Self {
        self.position = position;
        self
    }
}

impl Default for ScrollButtons {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ScrollButtons {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let button_color = self.button_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.2,
            a: 0.9,
        });
        let icon_color = self.icon_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.7,
            a: 1.0,
        });

        let button = |icon: &str| {
            div()
                .w(px(self.button_size))
                .h(px(self.button_size))
                .bg(button_color)
                .rounded_full()
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .hover(|s| s.bg(Hsla {
                    h: 0.0,
                    s: 0.0,
                    l: 0.3,
                    a: 0.9,
                }))
                .child(
                    div()
                        .text_size(px(14.0))
                        .text_color(icon_color)
                        .child(icon.to_string()),
                )
        };

        let mut container = div().flex().flex_col().gap_2();

        container = match self.position {
            ScrollButtonPosition::Right => container.items_end(),
            ScrollButtonPosition::Left => container.items_start(),
            ScrollButtonPosition::Center => container.items_center(),
        };

        if self.show_top {
            container = container.child(button("↑"));
        }

        if self.show_bottom {
            container = container.child(button("↓"));
        }

        container
    }
}
