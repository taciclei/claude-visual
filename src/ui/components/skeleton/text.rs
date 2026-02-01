//! Skeleton component for text blocks

use crate::app::state::AppState;
use crate::ui::pct;
use gpui::prelude::*;
use gpui::*;
use std::sync::Arc;

/// Skeleton for text blocks (multiple lines)
pub struct SkeletonText {
    app_state: Arc<AppState>,
    /// Number of lines
    lines: usize,
    /// Line height
    line_height: f32,
    /// Gap between lines
    gap: f32,
    /// Last line width percentage (0.0 - 1.0)
    last_line_width: f32,
}

impl SkeletonText {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            lines: 3,
            line_height: 12.0,
            gap: 8.0,
            last_line_width: 0.6,
        }
    }

    /// Set number of lines
    pub fn set_lines(&mut self, lines: usize, cx: &mut Context<Self>) {
        self.lines = lines;
        cx.notify();
    }

    /// Set line height
    pub fn set_line_height(&mut self, height: f32, cx: &mut Context<Self>) {
        self.line_height = height;
        cx.notify();
    }

    /// Set gap between lines
    pub fn set_gap(&mut self, gap: f32, cx: &mut Context<Self>) {
        self.gap = gap;
        cx.notify();
    }

    /// Set last line width (0.0 - 1.0)
    pub fn set_last_line_width(&mut self, width: f32, cx: &mut Context<Self>) {
        self.last_line_width = width.clamp(0.0, 1.0);
        cx.notify();
    }
}

impl Render for SkeletonText {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let base_color = theme.colors.surface_hover;

        div()
            .id("skeleton-text")
            .flex()
            .flex_col()
            .gap(px(self.gap))
            .w_full()
            .children((0..self.lines).map(|i| {
                let is_last = i == self.lines - 1;
                let width_pct = if is_last {
                    self.last_line_width * 100.0
                } else {
                    100.0
                };

                div()
                    .h(px(self.line_height))
                    .w(pct(width_pct))
                    .rounded(px(2.0))
                    .bg(base_color)
            }))
    }
}
