//! Credit card CVV input component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

#[derive(IntoElement)]
pub struct CvvInput {
    id: ElementId,
    value: SharedString,
    masked: bool,
    state: OtpState,
    disabled: bool,
}

impl CvvInput {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: "".into(),
            masked: true,
            state: OtpState::default(),
            disabled: false,
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

    pub fn state(mut self, state: OtpState) -> Self {
        self.state = state;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for CvvInput {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let border_color = match self.state {
            OtpState::Default => hsla(0.0, 0.0, 0.3, 1.0),
            OtpState::Focused => hsla(0.6, 0.7, 0.5, 1.0),
            OtpState::Success => hsla(0.35, 0.7, 0.45, 1.0),
            OtpState::Error => hsla(0.0, 0.7, 0.5, 1.0),
        };

        let display_value = if self.masked {
            self.value.chars().map(|_| '•').collect::<String>()
        } else {
            self.value.to_string()
        };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(
                div()
                    .w(px(60.0))
                    .h(px(40.0))
                    .px(px(12.0))
                    .flex()
                    .items_center()
                    .rounded(px(6.0))
                    .border_1()
                    .border_color(border_color)
                    .bg(hsla(0.0, 0.0, 0.12, 1.0))
                    .when(self.disabled, |el| el.opacity(0.5))
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(gpui::FontWeight::MEDIUM)
                            .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                            .child(display_value),
                    ),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                    .child("3-4 digits"),
            )
    }
}
