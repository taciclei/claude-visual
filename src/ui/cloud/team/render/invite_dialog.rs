//! Invite member dialog rendering

use gpui::prelude::*;
use gpui::*;

use super::super::TeamPanel;
use crate::cloud::team::TeamRole;

impl TeamPanel {
    /// Render invite dialog
    pub(crate) fn render_invite_dialog(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let surface = theme.colors.surface;
        let border = theme.colors.border;
        let surface_hover = theme.colors.surface_hover;
        let text = theme.colors.text;
        let text_muted = theme.colors.text_muted;
        let background = theme.colors.background;
        let accent = theme.colors.accent;
        let accent_hover = theme.colors.accent_hover;

        let close_listener = cx.listener(|this, _, _window, cx| {
            this.close_invite_dialog(cx);
        });
        let cancel_listener = cx.listener(|this, _, _window, cx| {
            this.close_invite_dialog(cx);
        });
        let send_listener = cx.listener(|this, _, _window, cx| {
            this.invite_member(cx);
        });

        div()
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .child(
                div()
                    .w(px(400.0))
                    .bg(surface)
                    .rounded_lg()
                    .border_1()
                    .border_color(border)
                    .overflow_hidden()
                    .child(
                        // Header
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .px_4()
                            .py_3()
                            .border_b_1()
                            .border_color(border)
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(text)
                                    .child("Invite Member"),
                            )
                            .child(
                                div()
                                    .id("close-invite-dialog")
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .hover(|s| s.bg(surface_hover))
                                    .cursor_pointer()
                                    .text_sm()
                                    .text_color(text_muted)
                                    .on_click(close_listener)
                                    .child("×"),
                            ),
                    )
                    .child(
                        // Content
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .px_4()
                            .py_4()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(div().text_sm().text_color(text).child("Email Address"))
                                    .child(
                                        div()
                                            .px_3()
                                            .py_2()
                                            .rounded_md()
                                            .bg(background)
                                            .border_1()
                                            .border_color(border)
                                            .text_sm()
                                            .text_color(if self.invite_email.is_empty() {
                                                text_muted
                                            } else {
                                                text
                                            })
                                            .child(if self.invite_email.is_empty() {
                                                "member@example.com".to_string()
                                            } else {
                                                self.invite_email.clone()
                                            }),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(div().text_sm().text_color(text).child("Role"))
                                    .child(
                                        div()
                                            .flex()
                                            .gap_2()
                                            .child(self.render_role_option(TeamRole::Viewer, cx))
                                            .child(self.render_role_option(TeamRole::Member, cx))
                                            .child(self.render_role_option(TeamRole::Admin, cx)),
                                    ),
                            ),
                    )
                    .child(
                        // Footer
                        div()
                            .flex()
                            .justify_end()
                            .gap_2()
                            .px_4()
                            .py_3()
                            .border_t_1()
                            .border_color(border)
                            .child(
                                div()
                                    .id("cancel-invite")
                                    .px_3()
                                    .py_2()
                                    .rounded_md()
                                    .bg(surface_hover)
                                    .hover(|s| s.opacity(0.8))
                                    .cursor_pointer()
                                    .text_sm()
                                    .text_color(text)
                                    .on_click(cancel_listener)
                                    .child("Cancel"),
                            )
                            .child(
                                div()
                                    .id("send-invite")
                                    .px_3()
                                    .py_2()
                                    .rounded_md()
                                    .bg(accent)
                                    .hover(|s| s.bg(accent_hover))
                                    .cursor_pointer()
                                    .text_sm()
                                    .text_color(text)
                                    .on_click(send_listener)
                                    .child("Send Invitation"),
                            ),
                    ),
            )
    }

    /// Render role option button
    pub(crate) fn render_role_option(
        &self,
        role: TeamRole,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let is_selected = self.invite_role == role;
        let accent = theme.colors.accent;
        let accent_hover = theme.colors.accent_hover;
        let surface = theme.colors.surface;
        let surface_hover = theme.colors.surface_hover;
        let text = theme.colors.text;

        let role_listener = cx.listener(move |this, _, _window, cx| {
            this.invite_role = role;
            cx.notify();
        });

        div()
            .id(ElementId::Name(format!("role-{:?}", role).into()))
            .px_3()
            .py_1()
            .rounded_md()
            .bg(if is_selected { accent } else { surface })
            .hover(|s| {
                s.bg(if is_selected {
                    accent_hover
                } else {
                    surface_hover
                })
            })
            .cursor_pointer()
            .text_sm()
            .text_color(text)
            .on_click(role_listener)
            .child(role.display_name())
    }
}
