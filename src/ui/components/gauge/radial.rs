//! Radial Chart component

use gpui::*;
use gpui::prelude::*;
use super::types::GaugeSize;

/// Radial bar chart with multiple segments
#[derive(IntoElement)]
pub struct RadialChart {
    id: ElementId,
    segments: Vec<RadialSegment>,
    size: GaugeSize,
    inner_radius_ratio: f32,
    gap_angle: f32,
    start_angle: f32,
    show_labels: bool,
    show_values: bool,
    background: gpui::Hsla,
}

/// Segment in a radial chart
#[derive(Debug, Clone)]
pub struct RadialSegment {
    pub value: f32,
    pub color: gpui::Hsla,
    pub label: Option<SharedString>,
}

impl RadialSegment {
    pub fn new(value: f32, color: gpui::Hsla) -> Self {
        Self {
            value,
            color,
            label: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl RadialChart {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            segments: Vec::new(),
            size: GaugeSize::Lg,
            inner_radius_ratio: 0.6,
            gap_angle: 2.0,
            start_angle: -90.0,
            show_labels: true,
            show_values: true,
            background: rgba(0x00000000).into(),
        }
    }

    pub fn segments(mut self, segments: Vec<RadialSegment>) -> Self {
        self.segments = segments;
        self
    }

    pub fn add_segment(mut self, segment: RadialSegment) -> Self {
        self.segments.push(segment);
        self
    }

    pub fn size(mut self, size: GaugeSize) -> Self {
        self.size = size;
        self
    }

    pub fn inner_radius_ratio(mut self, ratio: f32) -> Self {
        self.inner_radius_ratio = ratio.clamp(0.0, 0.95);
        self
    }

    pub fn gap_angle(mut self, angle: f32) -> Self {
        self.gap_angle = angle;
        self
    }

    pub fn start_angle(mut self, angle: f32) -> Self {
        self.start_angle = angle;
        self
    }

    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    pub fn show_values(mut self, show: bool) -> Self {
        self.show_values = show;
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = color;
        self
    }

    pub fn total(&self) -> f32 {
        self.segments.iter().map(|s| s.value).sum()
    }
}

impl RenderOnce for RadialChart {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let diameter = self.size.diameter();
        let total = self.total();

        div()
            .id(self.id)
            .relative()
            .size(px(diameter))
            .bg(self.background)
            .flex()
            .items_center()
            .justify_center()
            .child(
                // Outer ring
                div()
                    .absolute()
                    .inset_0()
                    .rounded_full()
                    .overflow_hidden()
                    .children(self.segments.iter().enumerate().map(|(i, segment)| {
                        let _percentage = if total > 0.0 {
                            segment.value / total
                        } else {
                            0.0
                        };
                        let height = diameter * (1.0 - (i as f32 * 0.15));
                        div()
                            .absolute()
                            .inset_0()
                            .m_auto()
                            .size(px(height))
                            .rounded_full()
                            .bg(segment.color.opacity(0.3))
                    })),
            )
            .child(
                // Center hole
                div()
                    .size(px(diameter * self.inner_radius_ratio))
                    .rounded_full()
                    .bg(self.background)
                    .flex()
                    .items_center()
                    .justify_center()
                    .when(self.show_values, |d| {
                        d.child(
                            div()
                                .text_size(px(self.size.font_size() * 0.6))
                                .font_weight(gpui::FontWeight::BOLD)
                                .text_color(rgba(0xffffffff))
                                .child(format!("{:.0}", total)),
                        )
                    }),
            )
    }
}
