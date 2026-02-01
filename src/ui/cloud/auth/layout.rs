//! Authentication dialog layout and main render implementation

use gpui::prelude::*;
use gpui::*;

use crate::cloud::{AuthState, OAuthProvider};

use super::dialog::AuthDialog;
use super::events::AuthDialogEvent;

impl Focusable for AuthDialog {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AuthDialog {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        let surface_hover = theme.colors.surface_hover;
        let background = theme.colors.background;
        let border = theme.colors.border;
        let text_muted = theme.colors.text_muted;

        let close_click = cx.listener(|_this, _event, _window, cx| {
            cx.emit(AuthDialogEvent::Closed);
        });

        // Modal backdrop
        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(Hsla::from(rgb(0x000000)).opacity(0.5))
            .child(
                // Modal content
                div()
                    .w(px(400.0))
                    .max_h(px(600.0))
                    .bg(background)
                    .rounded_xl()
                    .border_1()
                    .border_color(border)
                    .shadow_xl()
                    .p_6()
                    .flex()
                    .flex_col()
                    // Close button
                    .child(
                        div().flex().flex_row().justify_end().child(
                            div()
                                .id("close-auth-dialog")
                                .size_6()
                                .rounded_full()
                                .flex()
                                .items_center()
                                .justify_center()
                                .cursor_pointer()
                                .hover(move |this| this.bg(surface_hover))
                                .on_click(close_click)
                                .child(div().text_lg().text_color(text_muted).child("×")),
                        ),
                    )
                    // Header
                    .child(self.render_header(cx))
                    // Error message
                    .child(self.render_error(cx))
                    // Loading state
                    .child(self.render_loading(cx))
                    // OAuth providers
                    .when(
                        !matches!(self.auth_state, AuthState::Authenticating),
                        |this| {
                            this.child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_3()
                                    .child(self.render_provider_button(OAuthProvider::GitHub, cx))
                                    .child(self.render_provider_button(OAuthProvider::Google, cx)),
                            )
                        },
                    )
                    // Divider
                    .child(self.render_divider(cx))
                    // Email form (simplified for now)
                    .child(
                        div().flex().flex_col().gap_3().child(
                            div()
                                .text_sm()
                                .text_color(theme.colors.text_muted)
                                .text_center()
                                .child("Email authentication coming soon"),
                        ),
                    )
                    // Mode toggle
                    .child(self.render_mode_toggle(cx)),
            )
    }
}

impl EventEmitter<AuthDialogEvent> for AuthDialog {}
