//! Password input with strength meter

use gpui::*;
use gpui::prelude::*;
use super::types::*;
use super::meter::PasswordStrengthMeter;
use super::requirements::PasswordRequirements;

/// Password input with strength meter
#[derive(IntoElement)]
pub struct PasswordInput {
    id: ElementId,
    value: SharedString,
    placeholder: SharedString,
    pub(crate) show_strength: bool,
    show_requirements: bool,
    pub(crate) visible: bool,
}

impl PasswordInput {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: "".into(),
            placeholder: "Enter password".into(),
            show_strength: true,
            show_requirements: true,
            visible: false,
        }
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn show_strength(mut self, show: bool) -> Self {
        self.show_strength = show;
        self
    }

    pub fn show_requirements(mut self, show: bool) -> Self {
        self.show_requirements = show;
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
}

impl RenderOnce for PasswordInput {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let has_value = !self.value.is_empty();
        let strength = PasswordStrength::from_password(&self.value);

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap(px(12.0))
            // Input field
            .child(
                div()
                    .flex()
                    .items_center()
                    .h(px(40.0))
                    .px(px(12.0))
                    .bg(hsla(0.0, 0.0, 0.08, 1.0))
                    .border_1()
                    .border_color(hsla(0.0, 0.0, 0.2, 1.0))
                    .rounded(px(8.0))
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(14.0))
                            .text_color(if has_value {
                                hsla(0.0, 0.0, 0.9, 1.0)
                            } else {
                                hsla(0.0, 0.0, 0.5, 1.0)
                            })
                            .child(if has_value {
                                if self.visible {
                                    self.value.to_string()
                                } else {
                                    "•".repeat(self.value.len())
                                }
                            } else {
                                self.placeholder.to_string()
                            })
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                            .cursor_pointer()
                            .child(if self.visible { "🙈" } else { "👁" })
                    )
            )
            // Strength meter
            .when(self.show_strength && has_value, |el| {
                el.child(
                    PasswordStrengthMeter::new("strength")
                        .strength(strength)
                        .variant(StrengthMeterVariant::Segments)
                )
            })
            // Requirements
            .when(self.show_requirements && has_value, |el| {
                el.child(
                    PasswordRequirements::new("reqs")
                        .from_password(&self.value)
                        .show_all(false)
                )
            })
    }
}
