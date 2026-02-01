//! Loading indicator component

use super::types::*;
use crate::app::state::AppState;
use gpui::prelude::*;
use gpui::*;
use std::sync::Arc;

/// Represents a loading state with optional message
pub struct LoadingIndicator {
    pub(crate) app_state: Arc<AppState>,
    /// Loading message
    pub(crate) message: Option<String>,
    /// Size variant
    pub(crate) size: LoadingSize,
}

impl LoadingIndicator {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            message: None,
            size: LoadingSize::default(),
        }
    }

    pub fn set_message(&mut self, message: Option<String>, cx: &mut Context<Self>) {
        self.message = message;
        cx.notify();
    }

    pub fn set_size(&mut self, size: LoadingSize, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }
}

impl Render for LoadingIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        let (spinner_size, text_size) = match self.size {
            LoadingSize::Small => (16.0, 10.0),
            LoadingSize::Medium => (24.0, 12.0),
            LoadingSize::Large => (32.0, 14.0),
        };

        div()
            .id("loading-indicator")
            .flex()
            .items_center()
            .gap_2()
            // Spinner
            .child(
                div()
                    .size(px(spinner_size))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(theme.colors.accent)
                    .child("◐"),
            )
            // Message
            .when_some(self.message.clone(), |this, msg| {
                this.child(
                    div()
                        .text_size(px(text_size))
                        .text_color(theme.colors.text_muted)
                        .child(msg),
                )
            })
    }
}
