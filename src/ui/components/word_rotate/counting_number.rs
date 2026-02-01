//! CountingNumber component - animated counting numbers

use gpui::*;
use gpui::prelude::*;

/// Counting number animation
#[derive(IntoElement)]
pub struct CountingNumber {
    id: ElementId,
    value: f64,
    decimals: usize,
    prefix: SharedString,
    suffix: SharedString,
    font_size: f32,
    font_weight: gpui::FontWeight,
    text_color: Option<gpui::Hsla>,
}

impl CountingNumber {
    pub fn new(id: impl Into<ElementId>, value: f64) -> Self {
        Self {
            id: id.into(),
            value,
            decimals: 0,
            prefix: "".into(),
            suffix: "".into(),
            font_size: 32.0,
            font_weight: gpui::FontWeight::BOLD,
            text_color: None,
        }
    }

    pub fn decimals(mut self, decimals: usize) -> Self {
        self.decimals = decimals;
        self
    }

    pub fn prefix(mut self, prefix: impl Into<SharedString>) -> Self {
        self.prefix = prefix.into();
        self
    }

    pub fn suffix(mut self, suffix: impl Into<SharedString>) -> Self {
        self.suffix = suffix.into();
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn font_weight(mut self, weight: gpui::FontWeight) -> Self {
        self.font_weight = weight;
        self
    }

    pub fn text_color(mut self, color: gpui::Hsla) -> Self {
        self.text_color = Some(color);
        self
    }
}

impl RenderOnce for CountingNumber {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.text_color.unwrap_or(hsla(0.0, 0.0, 0.95, 1.0));

        let formatted = if self.decimals > 0 {
            format!("{:.1$}", self.value, self.decimals)
        } else {
            format!("{}", self.value as i64)
        };

        div()
            .id(self.id)
            .flex()
            .items_baseline()
            .child(
                div()
                    .text_size(px(self.font_size))
                    .font_weight(self.font_weight)
                    .text_color(color)
                    .child(format!("{}{}{}", self.prefix, formatted, self.suffix))
            )
    }
}
