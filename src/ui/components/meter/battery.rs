//! Battery indicator component

use super::types::MeterSize;
use crate::ui::pct;
use gpui::prelude::*;
use gpui::*;

/// Battery indicator component
#[derive(IntoElement)]
pub struct BatteryIndicator {
    id: ElementId,
    level: f32,
    charging: bool,
    size: MeterSize,
    show_percentage: bool,
}

impl BatteryIndicator {
    pub fn new(id: impl Into<ElementId>, level: f32) -> Self {
        Self {
            id: id.into(),
            level: level.clamp(0.0, 100.0),
            charging: false,
            size: MeterSize::default(),
            show_percentage: true,
        }
    }

    pub fn charging(mut self, charging: bool) -> Self {
        self.charging = charging;
        self
    }

    pub fn size(mut self, size: MeterSize) -> Self {
        self.size = size;
        self
    }

    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }

    pub(super) fn get_color(&self) -> gpui::Hsla {
        if self.charging {
            hsla(0.35, 0.7, 0.45, 1.0)
        } else if self.level <= 10.0 {
            hsla(0.0, 0.7, 0.5, 1.0)
        } else if self.level <= 20.0 {
            hsla(0.12, 0.8, 0.5, 1.0)
        } else {
            hsla(0.35, 0.7, 0.45, 1.0)
        }
    }
}

impl RenderOnce for BatteryIndicator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.get_color();
        let (width, height) = match self.size {
            MeterSize::Small => (20.0, 10.0),
            MeterSize::Medium => (30.0, 14.0),
            MeterSize::Large => (40.0, 18.0),
        };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(4.0))
            .child(
                // Battery body
                div()
                    .w(px(width))
                    .h(px(height))
                    .rounded(px(2.0))
                    .border_1()
                    .border_color(hsla(0.0, 0.0, 0.4, 1.0))
                    .p(px(2.0))
                    .flex()
                    .child(div().h_full().w(pct(self.level)).rounded(px(1.0)).bg(color)),
            )
            .child(
                // Battery tip
                div()
                    .w(px(2.0))
                    .h(px(height * 0.5))
                    .rounded_r(px(1.0))
                    .bg(hsla(0.0, 0.0, 0.4, 1.0)),
            )
            .when(self.charging, |el| {
                el.child(
                    div()
                        .text_size(px(height * 0.8))
                        .text_color(hsla(0.12, 0.8, 0.5, 1.0))
                        .child("⚡"),
                )
            })
            .when(self.show_percentage, |el| {
                el.child(
                    div()
                        .text_size(px(height * 0.7))
                        .text_color(hsla(0.0, 0.0, 0.6, 1.0))
                        .child(format!("{}%", self.level as u32)),
                )
            })
    }
}
