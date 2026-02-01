//! Authentication button components

use gpui::prelude::*;
use gpui::*;

use crate::cloud::OAuthProvider;

use super::dialog::{AuthDialog, AuthMode};
use super::events::AuthDialogEvent;

impl AuthDialog {
    /// Render OAuth provider button
    pub(super) fn render_provider_button(
        &self,
        provider: OAuthProvider,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let is_loading = matches!(self.auth_state, crate::cloud::AuthState::Authenticating);

        let surface_color = theme.colors.surface;
        let text_color = theme.colors.text;

        let on_click = cx.listener(move |_this, _event, _window, cx| {
            cx.emit(AuthDialogEvent::SignIn(provider));
        });

        div()
            .id(ElementId::Name(
                format!("auth-provider-{:?}", provider).into(),
            ))
            .w_full()
            .py_2()
            .px_4()
            .rounded_lg()
            .cursor_pointer()
            .bg(match provider {
                OAuthProvider::GitHub => Hsla::from(rgb(0x24292e)),
                OAuthProvider::Google => Hsla::from(rgb(0xffffff)),
                OAuthProvider::Email => surface_color,
            })
            .border_1()
            .border_color(match provider {
                OAuthProvider::Google => Hsla::from(rgb(0xdadce0)),
                _ => Hsla::transparent_black(),
            })
            .when(!is_loading, |this| this.hover(|this| this.opacity(0.9)))
            .when(is_loading, |this| this.opacity(0.6).cursor_not_allowed())
            .on_click(on_click)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .gap_3()
                    .child(div().text_base().child(match provider {
                        OAuthProvider::GitHub => "GitHub",
                        OAuthProvider::Google => "Google",
                        OAuthProvider::Email => "Email",
                    }))
                    .text_color(match provider {
                        OAuthProvider::GitHub => Hsla::from(rgb(0xffffff)),
                        OAuthProvider::Google => Hsla::from(rgb(0x000000)),
                        OAuthProvider::Email => text_color,
                    }),
            )
    }

    /// Render mode toggle
    pub(super) fn render_mode_toggle(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let text_muted = theme.colors.text_muted;
        let accent = theme.colors.accent;

        let on_click = cx.listener(|this, _event, _window, cx| {
            this.toggle_mode(cx);
        });

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .gap_1()
            .mt_4()
            .child(
                div()
                    .text_sm()
                    .text_color(text_muted)
                    .child(match self.auth_mode {
                        AuthMode::SignIn => "Don't have an account?",
                        AuthMode::SignUp => "Already have an account?",
                    }),
            )
            .child(
                div()
                    .id("toggle-auth-mode")
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(accent)
                    .cursor_pointer()
                    .hover(|this| this.opacity(0.8))
                    .on_click(on_click)
                    .child(match self.auth_mode {
                        AuthMode::SignIn => "Sign up",
                        AuthMode::SignUp => "Sign in",
                    }),
            )
    }
}
