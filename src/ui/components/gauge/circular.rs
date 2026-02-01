//! Circular Progress component

use super::types::GaugeSize;
use gpui::prelude::*;
use gpui::*;

/// Circular progress with percentage
#[derive(IntoElement)]
pub struct CircularProgress {
    id: ElementId,
    value: f32,
    max: f32,
    size: GaugeSize,
    thickness: Option<f32>,
    pub show_percentage: bool,
    show_label: bool,
    label: Option<SharedString>,
    track_color: gpui::Hsla,
    progress_color: gpui::Hsla,
    text_color: gpui::Hsla,
    clockwise: bool,
}

impl CircularProgress {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: 0.0,
            max: 100.0,
            size: GaugeSize::default(),
            thickness: None,
            show_percentage: true,
            show_label: false,
            label: None,
            track_color: rgba(0x8888881a).into(),
            progress_color: rgb(0x3b82f6).into(),
            text_color: rgba(0xffffffff).into(),
            clockwise: true,
        }
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    pub fn percentage(mut self, pct: f32) -> Self {
        self.value = pct;
        self.max = 100.0;
        self
    }

    pub fn size(mut self, size: GaugeSize) -> Self {
        self.size = size;
        self
    }

    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = Some(thickness);
        self
    }

    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }

    pub fn show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self.show_label = true;
        self
    }

    pub fn track_color(mut self, color: gpui::Hsla) -> Self {
        self.track_color = color;
        self
    }

    pub fn progress_color(mut self, color: gpui::Hsla) -> Self {
        self.progress_color = color;
        self
    }

    pub fn text_color(mut self, color: gpui::Hsla) -> Self {
        self.text_color = color;
        self
    }

    pub fn clockwise(mut self, clockwise: bool) -> Self {
        self.clockwise = clockwise;
        self
    }

    pub fn normalized(&self) -> f32 {
        if self.max == 0.0 {
            return 0.0;
        }
        (self.value / self.max).clamp(0.0, 1.0)
    }
}

impl RenderOnce for CircularProgress {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let diameter = self.size.diameter();
        let stroke = self.thickness.unwrap_or(self.size.stroke_width());
        let font_size = self.size.font_size();
        let progress = self.normalized();
        let percentage = (progress * 100.0).round() as i32;

        div()
            .id(self.id)
            .relative()
            .size(px(diameter))
            .flex()
            .items_center()
            .justify_center()
            .child(
                // Track circle
                div()
                    .absolute()
                    .inset_0()
                    .rounded_full()
                    .border(px(stroke))
                    .border_color(self.track_color),
            )
            .child(
                // Progress indicator (visual approximation)
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
                            .bg(self.progress_color.opacity(0.3)),
                    ),
            )
            .child(
                // Center content
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .when(self.show_percentage, |d| {
                        d.child(
                            div()
                                .text_size(px(font_size * 0.8))
                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                .text_color(self.text_color)
                                .child(format!("{}%", percentage)),
                        )
                    })
                    .when(self.show_label, |d| {
                        d.when_some(self.label.clone(), |d, label| {
                            d.child(
                                div()
                                    .text_size(px(font_size * 0.35))
                                    .text_color(self.text_color.opacity(0.7))
                                    .child(label),
                            )
                        })
                    }),
            )
    }
}
