//! PIN Input component - for numeric PINs

use super::types::*;
use gpui::prelude::*;
use gpui::*;

#[derive(IntoElement)]
pub struct PinInput {
    id: ElementId,
    length: usize,
    value: SharedString,
    masked: bool,
    size: OtpSize,
    state: OtpState,
    disabled: bool,
    show_toggle: bool,
}

impl PinInput {
    pub fn new(id: impl Into<ElementId>, length: usize) -> Self {
        Self {
            id: id.into(),
            length,
            value: "".into(),
            masked: true,
            size: OtpSize::default(),
            state: OtpState::default(),
            disabled: false,
            show_toggle: true,
        }
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    pub fn masked(mut self, masked: bool) -> Self {
        self.masked = masked;
        self
    }

    pub fn size(mut self, size: OtpSize) -> Self {
        self.size = size;
        self
    }

    pub fn state(mut self, state: OtpState) -> Self {
        self.state = state;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn show_toggle(mut self, show: bool) -> Self {
        self.show_toggle = show;
        self
    }
}

impl RenderOnce for PinInput {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (dot_size, gap) = match self.size {
            OtpSize::Small => (12.0, 16.0),
            OtpSize::Medium => (16.0, 20.0),
            OtpSize::Large => (20.0, 24.0),
        };

        let chars: Vec<char> = self.value.chars().collect();

        let state_color = match self.state {
            OtpState::Default => hsla(0.0, 0.0, 0.4, 1.0),
            OtpState::Focused => hsla(0.6, 0.7, 0.5, 1.0),
            OtpState::Success => hsla(0.35, 0.7, 0.45, 1.0),
            OtpState::Error => hsla(0.0, 0.7, 0.5, 1.0),
        };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(12.0))
            .when(self.disabled, |el| el.opacity(0.5))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(gap))
                    .children((0..self.length).map(|i| {
                        let has_value = i < chars.len();

                        if self.masked {
                            // Show dots
                            div().size(px(dot_size)).rounded_full().bg(if has_value {
                                state_color
                            } else {
                                hsla(0.0, 0.0, 0.25, 1.0)
                            })
                        } else {
                            // Show numbers
                            div()
                                .size(px(dot_size * 2.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded(px(4.0))
                                .border_1()
                                .border_color(if has_value {
                                    state_color
                                } else {
                                    hsla(0.0, 0.0, 0.25, 1.0)
                                })
                                .when(has_value, |el| {
                                    el.child(
                                        div()
                                            .text_size(px(dot_size))
                                            .font_weight(gpui::FontWeight::SEMIBOLD)
                                            .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                                            .child(chars[i].to_string()),
                                    )
                                })
                        }
                    })),
            )
            .when(self.show_toggle, |el| {
                el.child(
                    div()
                        .ml(px(8.0))
                        .cursor_pointer()
                        .text_size(px(dot_size))
                        .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                        .child(if self.masked { "👁" } else { "🔒" }),
                )
            })
    }
}
