//! Speedometer gauge

use gpui::prelude::*;
use gpui::*;

/// Speedometer gauge
#[derive(IntoElement)]
pub struct Speedometer {
    id: ElementId,
    value: f32,
    max: f32,
    unit: SharedString,
    size: f32,
    danger_threshold: Option<f32>,
}

impl Speedometer {
    pub fn new(id: impl Into<ElementId>, value: f32) -> Self {
        Self {
            id: id.into(),
            value,
            max: 100.0,
            unit: "".into(),
            size: 150.0,
            danger_threshold: None,
        }
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    pub fn unit(mut self, unit: impl Into<SharedString>) -> Self {
        self.unit = unit.into();
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn danger_threshold(mut self, threshold: f32) -> Self {
        self.danger_threshold = Some(threshold);
        self
    }

    fn get_color(&self) -> gpui::Hsla {
        if let Some(threshold) = self.danger_threshold {
            if self.value >= threshold {
                return hsla(0.0, 0.7, 0.5, 1.0);
            }
        }
        hsla(0.6, 0.7, 0.5, 1.0)
    }
}

impl RenderOnce for Speedometer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.get_color();
        let _percentage = (self.value / self.max).clamp(0.0, 1.0);

        div()
            .id(self.id)
            .size(px(self.size))
            .rounded_full()
            .bg(hsla(0.0, 0.0, 0.12, 1.0))
            .border_4()
            .border_color(color)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(4.0))
            .child(
                div()
                    .text_size(px(self.size / 3.5))
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(hsla(0.0, 0.0, 0.95, 1.0))
                    .child(format!("{:.0}", self.value)),
            )
            .child(
                div()
                    .text_size(px(self.size / 10.0))
                    .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                    .child(self.unit.clone()),
            )
    }
}
