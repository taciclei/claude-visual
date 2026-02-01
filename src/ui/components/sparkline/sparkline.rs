//! Main sparkline component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Sparkline component - inline mini chart
#[derive(IntoElement)]
pub struct Sparkline {
    id: ElementId,
    pub(crate) data: Vec<f64>,
    pub(crate) variant: SparklineVariant,
    size: SparklineSize,
    color: Option<gpui::Hsla>,
    pub(crate) show_min_max: bool,
    show_last_value: bool,
    animate: bool,
}

impl Sparkline {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            data: Vec::new(),
            variant: SparklineVariant::default(),
            size: SparklineSize::default(),
            color: None,
            show_min_max: false,
            show_last_value: false,
            animate: true,
        }
    }

    pub fn data(mut self, data: Vec<f64>) -> Self {
        self.data = data;
        self
    }

    pub fn variant(mut self, variant: SparklineVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: SparklineSize) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: gpui::Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn show_min_max(mut self, show: bool) -> Self {
        self.show_min_max = show;
        self
    }

    pub fn show_last_value(mut self, show: bool) -> Self {
        self.show_last_value = show;
        self
    }

    pub fn animate(mut self, animate: bool) -> Self {
        self.animate = animate;
        self
    }
}

impl RenderOnce for Sparkline {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or(hsla(0.6, 0.7, 0.5, 1.0));
        let height = self.size.height();
        let width = self.size.width();

        // Calculate min/max for scaling
        let min_val = self.data.iter().copied().fold(f64::INFINITY, f64::min);
        let max_val = self.data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let range = (max_val - min_val).max(0.001);

        let bar_count = self.data.len().max(1);
        let bar_width = width / bar_count as f32;

        div()
            .id(self.id)
            .flex()
            .items_end()
            .gap(px(1.0))
            .h(px(height))
            .w(px(width))
            .children(self.data.iter().enumerate().map(|(i, &value)| {
                let normalized = ((value - min_val) / range) as f32;
                let bar_height = (normalized * height).max(2.0);

                let is_last = i == self.data.len() - 1;
                let bar_color = if is_last && self.show_last_value {
                    hsla(color.h, color.s, color.l + 0.1, 1.0)
                } else {
                    color
                };

                div()
                    .w(px(bar_width - 1.0))
                    .h(px(bar_height))
                    .bg(bar_color)
                    .rounded(px(1.0))
            }))
    }
}
