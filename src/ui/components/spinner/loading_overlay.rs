//! Loading overlay for containers

use gpui::*;
use gpui::prelude::*;
use super::types::*;
use super::spinner::Spinner;

/// Loading overlay for containers
#[derive(Clone)]
pub struct LoadingOverlay {
    message: Option<String>,
    spinner_size: SpinnerSize,
    blur_background: bool,
}

impl LoadingOverlay {
    pub fn new() -> Self {
        Self {
            message: None,
            spinner_size: SpinnerSize::Large,
            blur_background: true,
        }
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn spinner_size(mut self, size: SpinnerSize) -> Self {
        self.spinner_size = size;
        self
    }

    pub fn blur_background(mut self, blur: bool) -> Self {
        self.blur_background = blur;
        self
    }
}

impl Default for LoadingOverlay {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for LoadingOverlay {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let backdrop = hsla(0.0, 0.0, 0.0, 0.7);
        let text = hsla(0.0, 0.0, 0.9, 1.0);

        div()
            .absolute()
            .inset_0()
            .bg(backdrop)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_3()
            .child(Spinner::new().size(self.spinner_size))
            .when_some(self.message, |d, msg| {
                d.child(
                    div()
                        .text_sm()
                        .text_color(text)
                        .child(msg)
                )
            })
    }
}
