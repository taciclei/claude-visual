//! Multi Gauge component

use super::types::GaugeSize;
use gpui::prelude::*;
use gpui::*;

/// Multi-ring gauge for comparing multiple values
#[derive(IntoElement)]
pub struct MultiGauge {
    id: ElementId,
    pub rings: Vec<GaugeRing>,
    size: GaugeSize,
    ring_gap: f32,
    show_legend: bool,
    background: gpui::Hsla,
}

/// Individual ring in a multi-gauge
#[derive(Debug, Clone)]
pub struct GaugeRing {
    pub value: f32,
    pub max: f32,
    pub color: gpui::Hsla,
    pub label: SharedString,
}

impl GaugeRing {
    pub fn new(value: f32, max: f32, color: gpui::Hsla, label: impl Into<SharedString>) -> Self {
        Self {
            value,
            max,
            color,
            label: label.into(),
        }
    }

    pub fn percentage(&self) -> f32 {
        if self.max == 0.0 {
            return 0.0;
        }
        (self.value / self.max * 100.0).clamp(0.0, 100.0)
    }
}

impl MultiGauge {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            rings: Vec::new(),
            size: GaugeSize::Lg,
            ring_gap: 8.0,
            show_legend: true,
            background: rgba(0x00000000).into(),
        }
    }

    pub fn rings(mut self, rings: Vec<GaugeRing>) -> Self {
        self.rings = rings;
        self
    }

    pub fn add_ring(mut self, ring: GaugeRing) -> Self {
        self.rings.push(ring);
        self
    }

    pub fn size(mut self, size: GaugeSize) -> Self {
        self.size = size;
        self
    }

    pub fn ring_gap(mut self, gap: f32) -> Self {
        self.ring_gap = gap;
        self
    }

    pub fn show_legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = color;
        self
    }
}

impl RenderOnce for MultiGauge {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let diameter = self.size.diameter();
        let ring_width = self.size.stroke_width();

        div()
            .id(self.id)
            .flex()
            .gap_4()
            .bg(self.background)
            .child(
                // Rings container
                div()
                    .relative()
                    .size(px(diameter))
                    .flex()
                    .items_center()
                    .justify_center()
                    .children(self.rings.iter().enumerate().map(|(i, ring)| {
                        let ring_diameter =
                            diameter - (i as f32 * (ring_width * 2.0 + self.ring_gap));
                        let progress = ring.percentage() / 100.0;

                        div()
                            .absolute()
                            .size(px(ring_diameter))
                            .rounded_full()
                            .border(px(ring_width))
                            .border_color(ring.color.opacity(0.2))
                            .child(
                                // Progress indicator
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
                                            .h(px(ring_diameter * progress))
                                            .bg(ring.color.opacity(0.4)),
                                    ),
                            )
                    })),
            )
            .when(self.show_legend, |d| {
                d.child(div().flex().flex_col().gap_2().justify_center().children(
                    self.rings.iter().map(|ring| {
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().size_3().rounded_full().bg(ring.color))
                            .child(div().text_sm().text_color(rgba(0xccccccff)).child(format!(
                                "{}: {:.0}%",
                                ring.label,
                                ring.percentage()
                            )))
                    }),
                ))
            })
    }
}
