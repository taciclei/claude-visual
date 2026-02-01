//! Members list view rendering

use gpui::*;
use gpui::prelude::*;

use crate::cloud::team::{Team, TeamRole};
use super::super::TeamPanel;

impl TeamPanel {
    /// Render members list
    pub(super) fn render_members(&self, team: &Team, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .flex()
            .flex_col()
            .px_4()
            .py_2()
            .child(
                // Invite button
                div()
                    .id("invite-member-btn")
                    .px_3()
                    .py_2()
                    .mb_2()
                    .rounded_md()
                    .bg(theme.colors.accent)
                    .hover(|s| s.bg(theme.colors.accent_hover))
                    .cursor_pointer()
                    .text_sm()
                    .text_color(theme.colors.text)
                    .text_center()
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.open_invite_dialog(cx);
                    }))
                    .child("+ Invite Member"),
            )
            .children(team.members.iter().map(|member| {
                let role_color = match member.role {
                    TeamRole::Owner => theme.colors.warning,
                    TeamRole::Admin => theme.colors.accent,
                    _ => theme.colors.text_muted,
                };

                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .hover(|s| s.bg(theme.colors.surface_hover))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                // Avatar
                                div()
                                    .size_8()
                                    .rounded_full()
                                    .bg(theme.colors.surface)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_sm()
                                    .text_color(theme.colors.text)
                                    .child(
                                        member
                                            .name
                                            .as_ref()
                                            .and_then(|n| n.chars().next())
                                            .or_else(|| member.email.chars().next())
                                            .unwrap_or('?')
                                            .to_string(),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(theme.colors.text)
                                            .child(member.name.clone().unwrap_or_else(|| member.email.clone())),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child(member.email.clone()),
                                    ),
                            ),
                    )
                    .child(
                        // Role badge
                        div()
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .bg(role_color.opacity(0.2))
                            .text_xs()
                            .text_color(role_color)
                            .child(member.role.display_name()),
                    )
            }))
    }
}
