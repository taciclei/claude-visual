//! Circular gauge component

use super::types::MeterVariant;
use gpui::prelude::*;
use gpui::*;

/// Circular gauge component
#[derive(IntoElement)]
pub struct CircularGauge {
    id: ElementId,
    value: f32,
    min: f32,
    max: f32,
    size: f32,
    thickness: f32,
    start_angle: f32,
    end_angle: f32,
    variant: MeterVariant,
    show_value: bool,
    label: Option<SharedString>,
    format: Option<SharedString>,
    background: Option<gpui::Hsla>,
    fill_color: Option<gpui::Hsla>,
}

impl CircularGauge {
    pub fn new(id: impl Into<ElementId>, value: f32) -> Self {
        Self {
            id: id.into(),
            value,
            min: 0.0,
            max: 100.0,
            size: 120.0,
            thickness: 10.0,
            start_angle: -135.0,
            end_angle: 135.0,
            variant: MeterVariant::default(),
            show_value: true,
            label: None,
            format: None,
            background: None,
            fill_color: None,
        }
    }

    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn start_angle(mut self, angle: f32) -> Self {
        self.start_angle = angle;
        self
    }

    pub fn end_angle(mut self, angle: f32) -> Self {
        self.end_angle = angle;
        self
    }

    pub fn variant(mut self, variant: MeterVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn format(mut self, format: impl Into<SharedString>) -> Self {
        self.format = Some(format.into());
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn fill_color(mut self, color: gpui::Hsla) -> Self {
        self.fill_color = Some(color);
        self
    }

    fn get_percentage(&self) -> f32 {
        ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0)
    }

    fn get_color(&self) -> gpui::Hsla {
        if let Some(color) = self.fill_color {
            return color;
        }

        match self.variant {
            MeterVariant::Default => hsla(0.6, 0.7, 0.5, 1.0),
            MeterVariant::Success => hsla(0.35, 0.7, 0.45, 1.0),
            MeterVariant::Warning => hsla(0.12, 0.8, 0.5, 1.0),
            MeterVariant::Danger => hsla(0.0, 0.7, 0.5, 1.0),
            MeterVariant::Info => hsla(0.55, 0.7, 0.5, 1.0),
            MeterVariant::Gradient => hsla(0.6, 0.7, 0.5, 1.0),
        }
    }
}

impl RenderOnce for CircularGauge {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let percentage = self.get_percentage();
        let _color = self.get_color();
        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.15, 1.0));

        let value_text = if let Some(format) = &self.format {
            format
                .to_string()
                .replace("{value}", &self.value.to_string())
        } else {
            format!("{:.0}", self.value)
        };

        // Simplified circular gauge using nested divs
        div()
            .id(self.id)
            .size(px(self.size))
            .rounded_full()
            .bg(bg)
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .size(px(self.size - self.thickness * 2.0))
                    .rounded_full()
                    .bg(hsla(0.0, 0.0, 0.1, 1.0))
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap(px(4.0))
                    .when(self.show_value, |el| {
                        el.child(
                            div()
                                .text_size(px(self.size / 4.0))
                                .font_weight(gpui::FontWeight::BOLD)
                                .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                                .child(value_text),
                        )
                    })
                    .when(self.label.is_some(), |el| {
                        el.child(
                            div()
                                .text_size(px(self.size / 10.0))
                                .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                                .child(self.label.unwrap_or_default()),
                        )
                    })
                    .child(
                        div()
                            .text_size(px(10.0))
                            .text_color(hsla(0.0, 0.0, 0.4, 1.0))
                            .child(format!("{:.0}%", percentage * 100.0)),
                    ),
            )
    }
}
