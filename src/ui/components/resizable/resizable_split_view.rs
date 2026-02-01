//! Resizable split view component

use gpui::*;
use gpui::prelude::*;
use super::types::*;
use super::resize_handle::ResizeHandle;

/// Resizable split view with two panels
#[derive(IntoElement)]
pub struct ResizableSplitView {
    first: Div,
    second: Div,
    direction: ResizeDirection,
    pub(crate) split_ratio: f32,
    handle_style: HandleStyle,
    pub(crate) min_first: f32,
    pub(crate) min_second: f32,
    handle_color: Option<Hsla>,
}

impl ResizableSplitView {
    pub fn new() -> Self {
        Self {
            first: div(),
            second: div(),
            direction: ResizeDirection::Horizontal,
            split_ratio: 0.5,
            handle_style: HandleStyle::Line,
            min_first: 100.0,
            min_second: 100.0,
            handle_color: None,
        }
    }

    pub fn first(mut self, content: impl IntoElement) -> Self {
        self.first = div().child(content);
        self
    }

    pub fn second(mut self, content: impl IntoElement) -> Self {
        self.second = div().child(content);
        self
    }

    pub fn direction(mut self, direction: ResizeDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn split_ratio(mut self, ratio: f32) -> Self {
        self.split_ratio = ratio.clamp(0.0, 1.0);
        self
    }

    pub fn handle_style(mut self, style: HandleStyle) -> Self {
        self.handle_style = style;
        self
    }

    pub fn min_first(mut self, min: f32) -> Self {
        self.min_first = min;
        self
    }

    pub fn min_second(mut self, min: f32) -> Self {
        self.min_second = min;
        self
    }

    pub fn handle_color(mut self, color: Hsla) -> Self {
        self.handle_color = Some(color);
        self
    }
}

impl Default for ResizableSplitView {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ResizableSplitView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let handle_color = self.handle_color.unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.25,
            a: 1.0,
        });

        let is_horizontal = matches!(self.direction, ResizeDirection::Horizontal);

        // For percentage-based sizing, we use flex with basis
        let first_flex = self.split_ratio;
        let second_flex = 1.0 - self.split_ratio;

        let mut container = div().flex().w_full().h_full();

        if is_horizontal {
            container = container.flex_row();
        } else {
            container = container.flex_col();
        }

        // First panel
        let first_panel = div()
            .flex_1()
            .min_w(px(self.min_first))
            .overflow_hidden()
            .child(self.first);

        // Handle
        let handle = ResizeHandle::new()
            .direction(self.direction)
            .style(self.handle_style)
            .color(handle_color);

        // Second panel
        let second_panel = div()
            .flex_1()
            .min_w(px(self.min_second))
            .overflow_hidden()
            .child(self.second);

        container
            .child(first_panel)
            .child(handle)
            .child(second_panel)
    }
}
