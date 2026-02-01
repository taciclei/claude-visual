//! Basic meter component

use gpui::*;
use gpui::prelude::*;
use crate::ui::pct;
use super::types::{MeterSize, MeterVariant, MeterOrientation};

/// Basic meter component - displays a value within a range
#[derive(IntoElement)]
pub struct Meter {
    id: ElementId,
    value: f32,
    min: f32,
    max: f32,
    low: Option<f32>,
    high: Option<f32>,
    optimum: Option<f32>,
    size: MeterSize,
    variant: MeterVariant,
    orientation: MeterOrientation,
    show_value: bool,
    label: Option<SharedString>,
    format: Option<SharedString>,
    background: Option<gpui::Hsla>,
    fill_color: Option<gpui::Hsla>,
}

impl Meter {
    pub fn new(id: impl Into<ElementId>, value: f32) -> Self {
        Self {
            id: id.into(),
            value,
            min: 0.0,
            max: 100.0,
            low: None,
            high: None,
            optimum: None,
            size: MeterSize::default(),
            variant: MeterVariant::default(),
            orientation: MeterOrientation::default(),
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

    pub fn low(mut self, low: f32) -> Self {
        self.low = Some(low);
        self
    }

    pub fn high(mut self, high: f32) -> Self {
        self.high = Some(high);
        self
    }

    pub fn optimum(mut self, optimum: f32) -> Self {
        self.optimum = Some(optimum);
        self
    }

    pub fn size(mut self, size: MeterSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: MeterVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn orientation(mut self, orientation: MeterOrientation) -> Self {
        self.orientation = orientation;
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

    pub(super) fn get_percentage(&self) -> f32 {
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

    fn get_height(&self) -> f32 {
        match self.size {
            MeterSize::Small => 6.0,
            MeterSize::Medium => 10.0,
            MeterSize::Large => 16.0,
        }
    }
}

impl RenderOnce for Meter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let percentage = self.get_percentage();
        let color = self.get_color();
        let height = self.get_height();
        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.2, 1.0));

        let is_vertical = self.orientation == MeterOrientation::Vertical;

        let value_text = if let Some(format) = &self.format {
            format.to_string().replace("{value}", &self.value.to_string())
        } else {
            format!("{:.0}%", percentage * 100.0)
        };

        let mut container = div().id(self.id).flex().gap(px(8.0));

        if is_vertical {
            container = container.flex_col().items_center();
        } else {
            container = container.flex_row().items_center();
        }

        // Label
        if let Some(label) = &self.label {
            container = container.child(
                div()
                    .text_size(px(12.0))
                    .text_color(hsla(0.0, 0.0, 0.6, 1.0))
                    .child(label.clone())
            );
        }

        // Meter bar
        let meter_bar = if is_vertical {
            div()
                .w(px(height))
                .h(px(100.0))
                .rounded(px(height / 2.0))
                .bg(bg)
                .overflow_hidden()
                .flex()
                .flex_col()
                .justify_end()
                .child(
                    div()
                        .w_full()
                        .h(pct(percentage * 100.0))
                        .rounded(px(height / 2.0))
                        .bg(color)
                )
        } else {
            div()
                .flex_1()
                .h(px(height))
                .rounded(px(height / 2.0))
                .bg(bg)
                .overflow_hidden()
                .child(
                    div()
                        .h_full()
                        .w(pct(percentage * 100.0))
                        .rounded(px(height / 2.0))
                        .bg(color)
                )
        };

        container = container.child(meter_bar);

        // Value display
        if self.show_value {
            container = container.child(
                div()
                    .text_size(px(12.0))
                    .font_weight(gpui::FontWeight::MEDIUM)
                    .text_color(hsla(0.0, 0.0, 0.8, 1.0))
                    .child(value_text)
            );
        }

        container
    }
}
