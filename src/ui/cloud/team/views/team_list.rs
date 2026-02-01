//! Team list view rendering

use gpui::*;
use gpui::prelude::*;

use super::super::{TeamPanel, TeamPanelEvent};

impl TeamPanel {
    /// Render team list
    pub(crate) fn render_team_list(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                // Header
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .py_2()
                    .border_b_1()
                    .border_color(theme.colors.border)
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.colors.text)
                            .child("Teams"),
                    )
                    .child(
                        div()
                            .id("create-team-btn")
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(theme.colors.accent)
                            .hover(|s| s.bg(theme.colors.accent_hover))
                            .cursor_pointer()
                            .text_xs()
                            .text_color(theme.colors.text)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.open_create_dialog(cx);
                            }))
                            .child("+ New Team"),
                    ),
            )
            .child(
                // Teams list
                div()
                    .flex_1()
                    .id("scroll-teams-list")
                    .overflow_y_scroll()
                    .px_2()
                    .py_2()
                    .children(self.teams.iter().enumerate().map(|(idx, team)| {
                        let is_selected = self.selected_team == Some(idx);
                        let bg = if is_selected {
                            theme.colors.accent
                        } else {
                            theme.colors.surface
                        };
                        let hover_bg = if is_selected {
                            theme.colors.accent_hover
                        } else {
                            theme.colors.surface_hover
                        };

                        div()
                            .id(ElementId::Name(format!("team-{}", idx).into()))
                            .px_3()
                            .py_2()
                            .rounded_md()
                            .bg(bg)
                            .hover(|s| s.bg(hover_bg))
                            .cursor_pointer()
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                this.select_team(idx, cx);
                            }))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        // Team avatar placeholder
                                        div()
                                            .size_8()
                                            .rounded_full()
                                            .bg(theme.colors.accent)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_sm()
                                            .text_color(theme.colors.text)
                                            .child(team.name.chars().next().unwrap_or('T').to_string()),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(theme.colors.text)
                                                    .child(team.name.clone()),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.colors.text_muted)
                                                    .child(format!("{} members", team.active_member_count())),
                                            ),
                                    ),
                            )
                    })),
            )
            .when(!self.invitations.is_empty(), |d| {
                d.child(
                    // Pending invitations section
                    div()
                        .px_4()
                        .py_2()
                        .border_t_1()
                        .border_color(theme.colors.border)
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.colors.text)
                                        .child("Invitations"),
                                )
                                .child(
                                    div()
                                        .px_1()
                                        .rounded_sm()
                                        .bg(theme.colors.warning)
                                        .text_xs()
                                        .text_color(theme.colors.text)
                                        .child(self.invitations.len().to_string()),
                                ),
                        )
                        .children(self.invitations.iter().map(|invite| {
                            let invite_id = invite.id.clone();
                            let invite_id2 = invite.id.clone();

                            div()
                                .px_2()
                                .py_2()
                                .rounded_md()
                                .bg(theme.colors.surface)
                                .mt_2()
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(theme.colors.text)
                                        .child(format!("{} invited you to join", invite.inviter_name.clone().unwrap_or_default())),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.colors.text)
                                        .child(invite.team_name.clone()),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .gap_2()
                                        .mt_2()
                                        .child(
                                            div()
                                                .id(ElementId::Name(format!("accept-{}", invite.id).into()))
                                                .px_2()
                                                .py_1()
                                                .rounded_md()
                                                .bg(theme.colors.success)
                                                .hover(|s| s.opacity(0.8))
                                                .cursor_pointer()
                                                .text_xs()
                                                .text_color(theme.colors.text)
                                                .on_click(cx.listener(move |_this, _, _window, cx| {
                                                    cx.emit(TeamPanelEvent::AcceptInvitation(invite_id.clone()));
                                                }))
                                                .child("Accept"),
                                        )
                                        .child(
                                            div()
                                                .id(ElementId::Name(format!("decline-{}", invite.id).into()))
                                                .px_2()
                                                .py_1()
                                                .rounded_md()
                                                .bg(theme.colors.error)
                                                .hover(|s| s.opacity(0.8))
                                                .cursor_pointer()
                                                .text_xs()
                                                .text_color(theme.colors.text)
                                                .on_click(cx.listener(move |_this, _, _window, cx| {
                                                    cx.emit(TeamPanelEvent::DeclineInvitation(invite_id2.clone()));
                                                }))
                                                .child("Decline"),
                                        )
                                )
                        })),
                )
            })
    }
}
