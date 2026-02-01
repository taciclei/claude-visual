//! Single keyboard key component

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Single keyboard key
#[derive(Clone, IntoElement)]
pub struct Kbd {
    /// Key text
    pub(crate) key: String,
    /// Size
    pub(crate) size: KbdSize,
    /// Style
    pub(crate) style: KbdStyle,
}

impl Kbd {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            size: KbdSize::default(),
            style: KbdStyle::default(),
        }
    }

    pub fn size(mut self, size: KbdSize) -> Self {
        self.size = size;
        self
    }

    pub fn style(mut self, style: KbdStyle) -> Self {
        self.style = style;
        self
    }

    // Common modifier keys
    pub fn cmd() -> Self {
        Self::new("⌘")
    }

    pub fn opt() -> Self {
        Self::new("⌥")
    }

    pub fn ctrl() -> Self {
        Self::new("⌃")
    }

    pub fn shift() -> Self {
        Self::new("⇧")
    }

    pub fn enter() -> Self {
        Self::new("↵")
    }

    pub fn tab() -> Self {
        Self::new("⇥")
    }

    pub fn esc() -> Self {
        Self::new("Esc")
    }

    pub fn space() -> Self {
        Self::new("Space")
    }

    pub fn delete() -> Self {
        Self::new("⌫")
    }

    pub fn up() -> Self {
        Self::new("↑")
    }

    pub fn down() -> Self {
        Self::new("↓")
    }

    pub fn left() -> Self {
        Self::new("←")
    }

    pub fn right() -> Self {
        Self::new("→")
    }
}

impl RenderOnce for Kbd {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.18, 1.0);
        let border = hsla(0.0, 0.0, 0.3, 1.0);
        let text = hsla(0.0, 0.0, 0.85, 1.0);

        let (px_h, py_v) = self.size.padding();
        let font_size = self.size.font_size();
        let min_width = self.size.min_width();

        let mut key = div()
            .min_w(px(min_width))
            .px(px(px_h))
            .py(px(py_v))
            .flex()
            .items_center()
            .justify_center()
            .text_color(text)
            .font_family("monospace");

        key = match self.style {
            KbdStyle::Default => key
                .bg(surface)
                .border_1()
                .border_color(border)
                .rounded(px(4.0))
                .shadow_sm(),
            KbdStyle::Flat => key
                .bg(surface)
                .rounded(px(4.0)),
            KbdStyle::Outline => key
                .border_1()
                .border_color(border)
                .rounded(px(4.0)),
            KbdStyle::Minimal => key
                .text_color(hsla(0.0, 0.0, 0.6, 1.0)),
        };

        key.child(self.key)
    }
}
