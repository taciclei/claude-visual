//! Basic Gauge component

use gpui::*;
use gpui::prelude::*;
use super::types::{GaugeStyle, GaugeSize, GaugeZone};

/// Gauge meter component
#[derive(IntoElement)]
pub struct Gauge {
    id: ElementId,
    pub value: f32,
    min: f32,
    max: f32,
    style: GaugeStyle,
    size: GaugeSize,
    pub zones: Vec<GaugeZone>,
    show_value: bool,
    show_ticks: bool,
    show_labels: bool,
    label: Option<SharedString>,
    unit: Option<SharedString>,
    format_value: Option<Box<dyn Fn(f32) -> String>>,
    background_color: gpui::Hsla,
    track_color: gpui::Hsla,
    value_color: gpui::Hsla,
    needle_color: Option<gpui::Hsla>,
}

impl Gauge {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: 0.0,
            min: 0.0,
            max: 100.0,
            style: GaugeStyle::default(),
            size: GaugeSize::default(),
            zones: Vec::new(),
            show_value: true,
            show_ticks: false,
            show_labels: false,
            label: None,
            unit: None,
            format_value: None,
            background_color: rgba(0x00000000).into(),
            track_color: rgba(0x8888881a).into(),
            value_color: rgb(0x3b82f6).into(),
            needle_color: None,
        }
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn style(mut self, style: GaugeStyle) -> Self {
        self.style = style;
        self
    }

    pub fn size(mut self, size: GaugeSize) -> Self {
        self.size = size;
        self
    }

    pub fn zones(mut self, zones: Vec<GaugeZone>) -> Self {
        self.zones = zones;
        self
    }

    pub fn add_zone(mut self, zone: GaugeZone) -> Self {
        self.zones.push(zone);
        self
    }

    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    pub fn show_ticks(mut self, show: bool) -> Self {
        self.show_ticks = show;
        self
    }

    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn unit(mut self, unit: impl Into<SharedString>) -> Self {
        self.unit = Some(unit.into());
        self
    }

    pub fn background_color(mut self, color: gpui::Hsla) -> Self {
        self.background_color = color;
        self
    }

    pub fn track_color(mut self, color: gpui::Hsla) -> Self {
        self.track_color = color;
        self
    }

    pub fn value_color(mut self, color: gpui::Hsla) -> Self {
        self.value_color = color;
        self
    }

    pub fn needle_color(mut self, color: gpui::Hsla) -> Self {
        self.needle_color = Some(color);
        self
    }

    /// Get the normalized value (0.0 to 1.0)
    pub fn normalized(&self) -> f32 {
        let range = self.max - self.min;
        if range == 0.0 {
            return 0.0;
        }
        ((self.value - self.min) / range).clamp(0.0, 1.0)
    }

    /// Get the color for the current value based on zones
    fn current_color(&self) -> gpui::Hsla {
        for zone in &self.zones {
            if self.value >= zone.from && self.value <= zone.to {
                return zone.color;
            }
        }
        self.value_color
    }

    fn format_display_value(&self) -> String {
        if let Some(ref formatter) = self.format_value {
            formatter(self.value)
        } else {
            format!("{:.0}", self.value)
        }
    }
}

impl RenderOnce for Gauge {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let diameter = self.size.diameter();
        let font_size = self.size.font_size();
        let progress = self.normalized();
        let current_color = self.current_color();
        let value_text = self.format_display_value();

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .items_center()
            .bg(self.background_color)
            .child(
                // Gauge visualization
                div()
                    .relative()
                    .size(px(diameter))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        // Track (background arc)
                        div()
                            .absolute()
                            .inset_0()
                            .rounded_full()
                            .border(px(self.size.stroke_width()))
                            .border_color(self.track_color)
                            .when(self.style == GaugeStyle::Semicircle, |d| {
                                d.h(px(diameter / 2.0))
                                    .rounded_t_full()
                                    .rounded_b_none()
                                    .border_b_0()
                            }),
                    )
                    .child(
                        // Progress arc indicator (simplified visual)
                        div()
                            .absolute()
                            .inset_0()
                            .rounded_full()
                            .overflow_hidden()
                            .child(
                                div()
                                    .absolute()
                                    .bottom_0()
                                    .left_0()
                                    .right_0()
                                    .h(px(diameter * progress))
                                    .bg(current_color.opacity(0.2)),
                            ),
                    )
                    .when(self.show_value, |d| {
                        d.child(
                            // Center value display
                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .child(
                                    div()
                                        .text_size(px(font_size))
                                        .font_weight(gpui::FontWeight::BOLD)
                                        .text_color(current_color)
                                        .child(value_text),
                                )
                                .when_some(self.unit.clone(), |d, unit| {
                                    d.child(
                                        div()
                                            .text_size(px(font_size * 0.4))
                                            .text_color(rgba(0x888888ff))
                                            .child(unit),
                                    )
                                }),
                        )
                    }),
            )
            .when_some(self.label, |d, label| {
                d.child(
                    div()
                        .mt_2()
                        .text_sm()
                        .text_color(rgba(0x888888ff))
                        .child(label),
                )
            })
    }
}
