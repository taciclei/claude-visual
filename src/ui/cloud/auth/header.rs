//! Authentication header components

use gpui::prelude::*;
use gpui::*;

use super::dialog::{AuthDialog, AuthMode};

impl AuthDialog {
    /// Render the header
    pub(super) fn render_header(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_2()
            .mb_6()
            .child(
                div()
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(text_color)
                    .child(match self.auth_mode {
                        AuthMode::SignIn => "Sign in to Claude Visual",
                        AuthMode::SignUp => "Create your account",
                    }),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(text_muted)
                    .child("Sync your conversations across devices"),
            )
    }

    /// Render divider
    pub(super) fn render_divider(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let border_color = theme.colors.border;
        let text_muted = theme.colors.text_muted;

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_4()
            .my_4()
            .child(div().flex_1().h(px(1.0)).bg(border_color))
            .child(
                div()
                    .text_xs()
                    .text_color(text_muted)
                    .child("or continue with email"),
            )
            .child(div().flex_1().h(px(1.0)).bg(border_color))
    }
}
