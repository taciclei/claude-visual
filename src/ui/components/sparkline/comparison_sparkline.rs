//! Comparison sparkline component

use gpui::*;
use gpui::prelude::*;

/// Comparison sparkline - shows two data series
#[derive(IntoElement)]
pub struct ComparisonSparkline {
    id: ElementId,
    pub(crate) primary_data: Vec<f64>,
    pub(crate) secondary_data: Vec<f64>,
    height: f32,
    width: f32,
    primary_color: Option<gpui::Hsla>,
    secondary_color: Option<gpui::Hsla>,
}

impl ComparisonSparkline {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            primary_data: Vec::new(),
            secondary_data: Vec::new(),
            height: 32.0,
            width: 80.0,
            primary_color: None,
            secondary_color: None,
        }
    }

    pub fn primary_data(mut self, data: Vec<f64>) -> Self {
        self.primary_data = data;
        self
    }

    pub fn secondary_data(mut self, data: Vec<f64>) -> Self {
        self.secondary_data = data;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn primary_color(mut self, color: gpui::Hsla) -> Self {
        self.primary_color = Some(color);
        self
    }

    pub fn secondary_color(mut self, color: gpui::Hsla) -> Self {
        self.secondary_color = Some(color);
        self
    }
}

impl RenderOnce for ComparisonSparkline {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let primary_color = self.primary_color.unwrap_or(hsla(0.6, 0.7, 0.5, 1.0));
        let secondary_color = self.secondary_color.unwrap_or(hsla(0.0, 0.0, 0.5, 0.5));

        let max_val = self.primary_data.iter().copied()
            .chain(self.secondary_data.iter().copied())
            .fold(f64::NEG_INFINITY, f64::max)
            .max(0.001);

        let count = self.primary_data.len().max(self.secondary_data.len()).max(1);
        let group_width = self.width / count as f32;
        let bar_width = (group_width - 2.0) / 2.0;

        div()
            .id(self.id)
            .flex()
            .items_end()
            .gap(px(2.0))
            .h(px(self.height))
            .w(px(self.width))
            .children((0..count).map(|i| {
                let primary = self.primary_data.get(i).copied().unwrap_or(0.0);
                let secondary = self.secondary_data.get(i).copied().unwrap_or(0.0);

                let primary_height = ((primary / max_val) as f32 * self.height).max(2.0);
                let secondary_height = ((secondary / max_val) as f32 * self.height).max(2.0);

                div()
                    .flex()
                    .items_end()
                    .gap(px(1.0))
                    .child(
                        div()
                            .w(px(bar_width))
                            .h(px(primary_height))
                            .bg(primary_color)
                            .rounded(px(1.0))
                    )
                    .child(
                        div()
                            .w(px(bar_width))
                            .h(px(secondary_height))
                            .bg(secondary_color)
                            .rounded(px(1.0))
                    )
            }))
    }
}
