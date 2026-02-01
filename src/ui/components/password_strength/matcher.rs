//! Password confirmation matcher component

use gpui::prelude::*;
use gpui::*;

/// Password confirmation matcher
#[derive(IntoElement)]
pub struct PasswordMatcher {
    id: ElementId,
    pub(crate) password: SharedString,
    pub(crate) confirmation: SharedString,
}

impl PasswordMatcher {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            password: "".into(),
            confirmation: "".into(),
        }
    }

    pub fn password(mut self, password: impl Into<SharedString>) -> Self {
        self.password = password.into();
        self
    }

    pub fn confirmation(mut self, confirmation: impl Into<SharedString>) -> Self {
        self.confirmation = confirmation.into();
        self
    }
}

impl RenderOnce for PasswordMatcher {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let is_empty = self.confirmation.is_empty();
        let matches = !is_empty && self.password == self.confirmation;

        div().id(self.id).when(!is_empty, |el| {
            let (icon, text, color) = if matches {
                ("✓", "Passwords match", hsla(0.35, 0.7, 0.45, 1.0))
            } else {
                ("✕", "Passwords don't match", hsla(0.0, 0.7, 0.5, 1.0))
            };

            el.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .child(div().text_size(px(12.0)).text_color(color).child(icon))
                    .child(div().text_size(px(12.0)).text_color(color).child(text)),
            )
        })
    }
}
