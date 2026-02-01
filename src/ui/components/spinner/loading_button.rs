//! Button with loading state

use super::spinner::Spinner;
use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Button with loading state
#[derive(Clone)]
pub struct LoadingButton {
    label: String,
    is_loading: bool,
    disabled: bool,
}

impl LoadingButton {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            is_loading: false,
            disabled: false,
        }
    }

    pub fn loading(mut self, is_loading: bool) -> Self {
        self.is_loading = is_loading;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for LoadingButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let accent = hsla(0.6, 0.8, 0.6, 1.0);

        let is_disabled = self.disabled || self.is_loading;

        div()
            .px_4()
            .py_2()
            .rounded(px(6.0))
            .bg(if is_disabled {
                accent.opacity(0.5)
            } else {
                accent
            })
            .flex()
            .items_center()
            .justify_center()
            .gap_2()
            .text_sm()
            .text_color(gpui::white())
            .cursor(if is_disabled {
                CursorStyle::default()
            } else {
                CursorStyle::PointingHand
            })
            .when(!is_disabled, |d| d.hover(|s| s.opacity(0.9)))
            .when(self.is_loading, |d| {
                d.child(Spinner::new().size(SpinnerSize::Small).color(gpui::white()))
            })
            .child(self.label)
    }
}
