//! Main scrollable area component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// A scrollable area with custom styled scrollbars
#[derive(IntoElement)]
pub struct ScrollArea {
    content: Div,
    width: Option<f32>,
    height: Option<f32>,
    max_height: Option<f32>,
    direction: ScrollDirection,
    scrollbar_visibility: ScrollbarVisibility,
    scrollbar_size: ScrollbarSize,
    scrollbar_color: Option<Hsla>,
    track_color: Option<Hsla>,
    padding: Option<f32>,
    border: bool,
    border_color: Option<Hsla>,
    background: Option<Hsla>,
}

impl ScrollArea {
    pub fn new() -> Self {
        Self {
            content: div(),
            width: None,
            height: None,
            max_height: None,
            direction: ScrollDirection::Vertical,
            scrollbar_visibility: ScrollbarVisibility::Auto,
            scrollbar_size: ScrollbarSize::Default,
            scrollbar_color: None,
            track_color: None,
            padding: None,
            border: false,
            border_color: None,
            background: None,
        }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.content = div().child(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.content = div().children(children);
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

    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = Some(max_height);
        self
    }

    pub fn direction(mut self, direction: ScrollDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn scrollbar_visibility(mut self, visibility: ScrollbarVisibility) -> Self {
        self.scrollbar_visibility = visibility;
        self
    }

    pub fn scrollbar_size(mut self, size: ScrollbarSize) -> Self {
        self.scrollbar_size = size;
        self
    }

    pub fn scrollbar_color(mut self, color: Hsla) -> Self {
        self.scrollbar_color = Some(color);
        self
    }

    pub fn track_color(mut self, color: Hsla) -> Self {
        self.track_color = Some(color);
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = Some(padding);
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

    pub fn background(mut self, color: Hsla) -> Self {
        self.background = Some(color);
        self
    }
}

impl Default for ScrollArea {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ScrollArea {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let _scrollbar_color = self.scrollbar_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.4,
            a: 0.6,
        });
        let border_color = self.border_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.3,
            a: 1.0,
        });

        let mut container = div().relative();

        if let Some(w) = self.width {
            container = container.w(px(w));
        } else {
            container = container.w_full();
        }

        if let Some(h) = self.height {
            container = container.h(px(h));
        }

        if let Some(mh) = self.max_height {
            container = container.max_h(px(mh));
        }

        // Apply overflow based on direction
        let mut container = match self.direction {
            ScrollDirection::Vertical => container.id("scroll-area-vertical").overflow_y_scroll(),
            ScrollDirection::Horizontal => {
                container.id("scroll-area-horizontal").overflow_x_scroll()
            }
            ScrollDirection::Both => container.id("scroll-area-both").overflow_scroll(),
        };

        if self.border {
            container = container.border_1().border_color(border_color);
        }

        if let Some(bg) = self.background {
            container = container.bg(bg);
        }

        let mut content = self.content;
        if let Some(p) = self.padding {
            content = content.p(px(p));
        }

        container.child(content)
    }
}
