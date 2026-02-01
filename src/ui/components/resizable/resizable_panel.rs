//! Resizable panel component

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// A resizable panel component
#[derive(IntoElement)]
pub struct ResizablePanel {
    content: Div,
    direction: ResizeDirection,
    handle_position: HandlePosition,
    handle_style: HandleStyle,
    pub(crate) min_size: f32,
    pub(crate) max_size: Option<f32>,
    pub(crate) default_size: f32,
    current_size: f32,
    pub(crate) collapsible: bool,
    collapsed: bool,
    handle_color: Option<Hsla>,
    handle_hover_color: Option<Hsla>,
    border: bool,
    border_color: Option<Hsla>,
}

impl ResizablePanel {
    pub fn new() -> Self {
        Self {
            content: div(),
            direction: ResizeDirection::Horizontal,
            handle_position: HandlePosition::End,
            handle_style: HandleStyle::Line,
            min_size: 100.0,
            max_size: None,
            default_size: 250.0,
            current_size: 250.0,
            collapsible: false,
            collapsed: false,
            handle_color: None,
            handle_hover_color: None,
            border: false,
            border_color: None,
        }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.content = div().child(child);
        self
    }

    pub fn direction(mut self, direction: ResizeDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn handle_position(mut self, position: HandlePosition) -> Self {
        self.handle_position = position;
        self
    }

    pub fn handle_style(mut self, style: HandleStyle) -> Self {
        self.handle_style = style;
        self
    }

    pub fn min_size(mut self, size: f32) -> Self {
        self.min_size = size;
        self
    }

    pub fn max_size(mut self, size: f32) -> Self {
        self.max_size = Some(size);
        self
    }

    pub fn default_size(mut self, size: f32) -> Self {
        self.default_size = size;
        self.current_size = size;
        self
    }

    pub fn current_size(mut self, size: f32) -> Self {
        self.current_size = size;
        self
    }

    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    pub fn handle_color(mut self, color: Hsla) -> Self {
        self.handle_color = Some(color);
        self
    }

    pub fn handle_hover_color(mut self, color: Hsla) -> Self {
        self.handle_hover_color = Some(color);
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
}

impl Default for ResizablePanel {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ResizablePanel {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let handle_color = self.handle_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.3,
            a: 1.0,
        });
        let handle_hover_color = self.handle_hover_color.unwrap_or(Hsla {
            h: 0.58,
            s: 0.7,
            l: 0.5,
            a: 1.0,
        });
        let border_color = self.border_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.25,
            a: 1.0,
        });

        let size = if self.collapsed { 0.0 } else { self.current_size };

        // Create resize handle
        let handle = |is_horizontal: bool| {
            let (width, height, cursor) = if is_horizontal {
                (4.0, f32::MAX, CursorStyle::ResizeLeftRight)
            } else {
                (f32::MAX, 4.0, CursorStyle::ResizeUpDown)
            };

            let handle_content = match self.handle_style {
                HandleStyle::Line => div()
                    .when(is_horizontal, |d| d.w(px(2.0)).h_full())
                    .when(!is_horizontal, |d| d.h(px(2.0)).w_full())
                    .bg(handle_color),
                HandleStyle::Dots => div()
                    .flex()
                    .when(is_horizontal, |d| d.flex_col())
                    .items_center()
                    .justify_center()
                    .gap(px(2.0))
                    .child(div().w(px(3.0)).h(px(3.0)).rounded_full().bg(handle_color))
                    .child(div().w(px(3.0)).h(px(3.0)).rounded_full().bg(handle_color))
                    .child(div().w(px(3.0)).h(px(3.0)).rounded_full().bg(handle_color)),
                HandleStyle::Grip => div()
                    .flex()
                    .when(is_horizontal, |d| d.flex_col())
                    .items_center()
                    .justify_center()
                    .gap(px(1.0))
                    .child(div().w(px(8.0)).h(px(1.0)).bg(handle_color))
                    .child(div().w(px(8.0)).h(px(1.0)).bg(handle_color))
                    .child(div().w(px(8.0)).h(px(1.0)).bg(handle_color)),
                HandleStyle::Hidden => div(),
            };

            div()
                .flex_shrink_0()
                .when(is_horizontal, |d| d.w(px(width)).h_full())
                .when(!is_horizontal, |d| d.h(px(height)).w_full())
                .flex()
                .items_center()
                .justify_center()
                .cursor(cursor)
                .hover(|s| s.bg(handle_hover_color))
                .child(handle_content)
        };

        let is_horizontal = matches!(self.direction, ResizeDirection::Horizontal | ResizeDirection::Both);

        let mut container = div().flex();

        // Set direction
        container = if is_horizontal {
            container.flex_row()
        } else {
            container.flex_col()
        };

        // Apply border
        if self.border {
            container = container.border_1().border_color(border_color);
        }

        // Add start handle if needed
        if matches!(self.handle_position, HandlePosition::Start | HandlePosition::Both) {
            container = container.child(handle(is_horizontal));
        }

        // Content with size
        let mut content_container = div().overflow_hidden().child(self.content);

        if is_horizontal {
            content_container = content_container.w(px(size)).h_full();
        } else {
            content_container = content_container.h(px(size)).w_full();
        }

        container = container.child(content_container);

        // Add end handle if needed
        if matches!(self.handle_position, HandlePosition::End | HandlePosition::Both) {
            container = container.child(handle(is_horizontal));
        }

        container
    }
}
