//! Password strength meter component

use super::types::*;
use crate::ui::pct;
use gpui::prelude::*;
use gpui::*;

/// Password strength meter component
#[derive(IntoElement)]
pub struct PasswordStrengthMeter {
    id: ElementId,
    strength: PasswordStrength,
    pub(crate) variant: StrengthMeterVariant,
    show_label: bool,
    total_segments: usize,
}

impl PasswordStrengthMeter {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            strength: PasswordStrength::default(),
            variant: StrengthMeterVariant::default(),
            show_label: true,
            total_segments: 5,
        }
    }

    pub fn strength(mut self, strength: PasswordStrength) -> Self {
        self.strength = strength;
        self
    }

    pub fn from_password(mut self, password: &str) -> Self {
        self.strength = PasswordStrength::from_password(password);
        self
    }

    pub fn variant(mut self, variant: StrengthMeterVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }

    pub fn total_segments(mut self, segments: usize) -> Self {
        self.total_segments = segments;
        self
    }
}

impl RenderOnce for PasswordStrengthMeter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.strength.color();
        let segments = self.strength.segments();

        match self.variant {
            StrengthMeterVariant::Bar => {
                let progress = (segments as f32 / self.total_segments as f32) * 100.0;

                div()
                    .id(self.id)
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .child(
                        div()
                            .w_full()
                            .h(px(4.0))
                            .bg(hsla(0.0, 0.0, 0.2, 1.0))
                            .rounded_full()
                            .child(div().h_full().w(pct(progress)).bg(color).rounded_full()),
                    )
                    .when(
                        self.show_label && self.strength != PasswordStrength::None,
                        |el| {
                            el.child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(color)
                                    .child(self.strength.label()),
                            )
                        },
                    )
            }
            StrengthMeterVariant::Segments => div()
                .id(self.id)
                .flex()
                .flex_col()
                .gap(px(6.0))
                .child(
                    div()
                        .flex()
                        .gap(px(4.0))
                        .children((0..self.total_segments).map(|i| {
                            let is_filled = i < segments;
                            div()
                                .flex_1()
                                .h(px(4.0))
                                .bg(if is_filled {
                                    color
                                } else {
                                    hsla(0.0, 0.0, 0.2, 1.0)
                                })
                                .rounded(px(2.0))
                        })),
                )
                .when(
                    self.show_label && self.strength != PasswordStrength::None,
                    |el| {
                        el.child(
                            div()
                                .text_size(px(12.0))
                                .text_color(color)
                                .child(self.strength.label()),
                        )
                    },
                ),
            StrengthMeterVariant::Circle => {
                let size = 48.0;
                let stroke_width = 4.0;
                let progress = segments as f32 / self.total_segments as f32;

                div()
                    .id(self.id)
                    .flex()
                    .items_center()
                    .gap(px(12.0))
                    .child(
                        div()
                            .relative()
                            .w(px(size))
                            .h(px(size))
                            .rounded_full()
                            .border(px(stroke_width))
                            .border_color(hsla(0.0, 0.0, 0.2, 1.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .text_color(color)
                                    .child(format!("{}%", (progress * 100.0) as i32)),
                            ),
                    )
                    .when(
                        self.show_label && self.strength != PasswordStrength::None,
                        |el| {
                            el.child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(gpui::FontWeight::MEDIUM)
                                    .text_color(color)
                                    .child(self.strength.label()),
                            )
                        },
                    )
            }
            StrengthMeterVariant::Text => {
                div()
                    .id(self.id)
                    .when(self.strength != PasswordStrength::None, |el| {
                        el.child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .child(div().w(px(8.0)).h(px(8.0)).rounded_full().bg(color))
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(color)
                                        .child(self.strength.label()),
                                ),
                        )
                    })
            }
        }
    }
}
