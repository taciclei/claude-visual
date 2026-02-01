//! Speed Gauge component

use gpui::*;
use gpui::prelude::*;
use super::types::{GaugeSize, GaugeZone};

/// Speed gauge / speedometer style component
#[derive(IntoElement)]
pub struct SpeedGauge {
    id: ElementId,
    pub value: f32,
    max_value: f32,
    size: GaugeSize,
    zones: Vec<GaugeZone>,
    show_needle: bool,
    show_speed_marks: bool,
    pub unit: SharedString,
    background: gpui::Hsla,
    needle_color: gpui::Hsla,
}

impl SpeedGauge {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: 0.0,
            max_value: 200.0,
            size: GaugeSize::Lg,
            zones: vec![
                GaugeZone::new(0.0, 60.0, rgb(0x22c55e).into()),
                GaugeZone::new(60.0, 120.0, rgb(0xeab308).into()),
                GaugeZone::new(120.0, 200.0, rgb(0xef4444).into()),
            ],
            show_needle: true,
            show_speed_marks: true,
            unit: "km/h".into(),
            background: rgba(0x1a1a1aff).into(),
            needle_color: rgb(0xef4444).into(),
        }
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn max_value(mut self, max: f32) -> Self {
        self.max_value = max;
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

    pub fn show_needle(mut self, show: bool) -> Self {
        self.show_needle = show;
        self
    }

    pub fn show_speed_marks(mut self, show: bool) -> Self {
        self.show_speed_marks = show;
        self
    }

    pub fn unit(mut self, unit: impl Into<SharedString>) -> Self {
        self.unit = unit.into();
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = color;
        self
    }

    pub fn needle_color(mut self, color: gpui::Hsla) -> Self {
        self.needle_color = color;
        self
    }
}

impl RenderOnce for SpeedGauge {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let diameter = self.size.diameter();
        let font_size = self.size.font_size();

        div()
            .id(self.id)
            .size(px(diameter))
            .rounded_full()
            .bg(self.background)
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(font_size))
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child(format!("{:.0}", self.value)),
                    )
                    .child(
                        div()
                            .text_size(px(font_size * 0.4))
                            .text_color(rgba(0xaaaaaaff))
                            .child(self.unit.clone()),
                    ),
            )
    }
}
