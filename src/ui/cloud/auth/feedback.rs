//! Authentication feedback components (error and loading states)

use gpui::*;
use gpui::prelude::*;

use crate::cloud::AuthState;

use super::dialog::AuthDialog;

impl AuthDialog {
    /// Render error message
    pub(super) fn render_error(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let error_color = theme.colors.error;

        div().when_some(self.error_message.clone(), move |this, message| {
            this.child(
                div()
                    .w_full()
                    .p_3()
                    .rounded_md()
                    .bg(error_color.opacity(0.1))
                    .border_1()
                    .border_color(error_color.opacity(0.3))
                    .child(
                        div()
                            .text_sm()
                            .text_color(error_color)
                            .child(message),
                    ),
            )
        })
    }

    /// Render loading state
    pub(super) fn render_loading(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let accent_color = theme.colors.accent;
        let accent_faded = accent_color.opacity(0.3);
        let text_muted = theme.colors.text_muted;

        div()
            .when(matches!(self.auth_state, AuthState::Authenticating), move |this| {
                this.child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_center()
                        .gap_2()
                        .py_2()
                        .child(
                            // Spinner animation would go here
                            div()
                                .size_4()
                                .rounded_full()
                                .border_2()
                                .border_color(accent_faded)
                                .border_color(accent_color),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(text_muted)
                                .child("Signing in..."),
                        ),
                )
            })
    }
}
