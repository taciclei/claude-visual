//! Trend indicator component

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Trend indicator component
#[derive(IntoElement)]
pub struct TrendIndicator {
    id: ElementId,
    pub(crate) direction: TrendDirection,
    pub(crate) value: Option<f64>,
    label: Option<SharedString>,
    size: TrendSize,
    show_icon: bool,
}

impl TrendIndicator {
    pub fn new(id: impl Into<ElementId>, direction: TrendDirection) -> Self {
        Self {
            id: id.into(),
            direction,
            value: None,
            label: None,
            size: TrendSize::default(),
            show_icon: true,
        }
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn size(mut self, size: TrendSize) -> Self {
        self.size = size;
        self
    }

    pub fn show_icon(mut self, show: bool) -> Self {
        self.show_icon = show;
        self
    }
}

impl RenderOnce for TrendIndicator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (color, icon) = match self.direction {
            TrendDirection::Up => (hsla(0.35, 0.7, 0.45, 1.0), "↑"),
            TrendDirection::Down => (hsla(0.0, 0.7, 0.5, 1.0), "↓"),
            TrendDirection::Flat => (hsla(0.0, 0.0, 0.5, 1.0), "→"),
        };

        let font_size = self.size.font_size();

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(4.0))
            .text_size(px(font_size))
            .text_color(color)
            .when(self.show_icon, |el| el.child(icon))
            .when_some(self.value, |el, val| {
                el.child(format!("{:.1}%", val.abs()))
            })
            .when_some(self.label, |el, label| {
                el.child(
                    div()
                        .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                        .child(label)
                )
            })
    }
}
