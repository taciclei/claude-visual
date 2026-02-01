//! Verification code input component with resend functionality

use super::types::*;
use gpui::prelude::*;
use gpui::*;

#[derive(IntoElement)]
pub struct VerificationInput {
    id: ElementId,
    length: usize,
    value: SharedString,
    state: OtpState,
    error_message: Option<SharedString>,
    resend_available: bool,
    resend_countdown: Option<u32>,
}

impl VerificationInput {
    pub fn new(id: impl Into<ElementId>, length: usize) -> Self {
        Self {
            id: id.into(),
            length,
            value: "".into(),
            state: OtpState::default(),
            error_message: None,
            resend_available: true,
            resend_countdown: None,
        }
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    pub fn state(mut self, state: OtpState) -> Self {
        self.state = state;
        self
    }

    pub fn error_message(mut self, message: impl Into<SharedString>) -> Self {
        self.error_message = Some(message.into());
        self.state = OtpState::Error;
        self
    }

    pub fn resend_available(mut self, available: bool) -> Self {
        self.resend_available = available;
        self
    }

    pub fn resend_countdown(mut self, seconds: u32) -> Self {
        self.resend_countdown = Some(seconds);
        self.resend_available = false;
        self
    }
}

impl RenderOnce for VerificationInput {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let chars: Vec<char> = self.value.chars().collect();

        let border_color = match self.state {
            OtpState::Default => hsla(0.0, 0.0, 0.3, 1.0),
            OtpState::Focused => hsla(0.6, 0.7, 0.5, 1.0),
            OtpState::Success => hsla(0.35, 0.7, 0.45, 1.0),
            OtpState::Error => hsla(0.0, 0.7, 0.5, 1.0),
        };

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap(px(16.0))
            .child(
                // OTP boxes
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap(px(8.0))
                    .children((0..self.length).map(|i| {
                        let char_at = chars.get(i).cloned();
                        let is_current = i == chars.len();

                        div()
                            .size(px(48.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(8.0))
                            .border_2()
                            .border_color(if is_current && self.state == OtpState::Focused {
                                hsla(0.6, 0.7, 0.5, 1.0)
                            } else {
                                border_color
                            })
                            .bg(hsla(0.0, 0.0, 0.12, 1.0))
                            .text_size(px(20.0))
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(hsla(0.0, 0.0, 0.95, 1.0))
                            .when(char_at.is_some(), |el| {
                                el.child(char_at.unwrap().to_string())
                            })
                            .when(is_current && self.state == OtpState::Focused, |el| {
                                el.child(div().w(px(2.0)).h(px(24.0)).bg(hsla(0.6, 0.7, 0.5, 1.0)))
                            })
                    })),
            )
            .when(self.error_message.is_some(), |el| {
                el.child(
                    div()
                        .text_size(px(13.0))
                        .text_color(hsla(0.0, 0.7, 0.5, 1.0))
                        .text_center()
                        .child(self.error_message.unwrap_or_default()),
                )
            })
            .child(
                // Resend section
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap(px(4.0))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                            .child("Didn't receive the code?"),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .when(self.resend_available, |el| {
                                el.text_color(hsla(0.6, 0.7, 0.5, 1.0))
                                    .cursor_pointer()
                                    .child("Resend")
                            })
                            .when(!self.resend_available, |el| {
                                el.text_color(hsla(0.0, 0.0, 0.4, 1.0)).child(format!(
                                    "Resend in {}s",
                                    self.resend_countdown.unwrap_or(0)
                                ))
                            }),
                    ),
            )
    }
}
