//! Team details view rendering

use gpui::*;
use gpui::prelude::*;

use crate::cloud::team::Team;
use super::super::{TeamPanel, TeamViewMode};

impl TeamPanel {
    /// Render team details
    pub(crate) fn render_team_details(&self, team: &Team, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .flex()
            .flex_col()
            .size_full()
            .child(
                // Header with back button
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_4()
                    .py_2()
                    .border_b_1()
                    .border_color(theme.colors.border)
                    .child(
                        div()
                            .id("back-to-teams")
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .hover(|s| s.bg(theme.colors.surface_hover))
                            .cursor_pointer()
                            .text_sm()
                            .text_color(theme.colors.text_muted)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.selected_team = None;
                                this.view_mode = TeamViewMode::TeamList;
                                cx.notify();
                            }))
                            .child("← Back"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.colors.text)
                            .child(team.name.clone()),
                    ),
            )
            .child(
                // Team info
                div()
                    .px_4()
                    .py_3()
                    .border_b_1()
                    .border_color(theme.colors.border)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                // Team avatar
                                div()
                                    .size(px(48.0))
                                    .rounded_lg()
                                    .bg(theme.colors.accent)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_lg()
                                    .text_color(theme.colors.text)
                                    .child(team.name.chars().next().unwrap_or('T').to_string()),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_base()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.colors.text)
                                            .child(team.name.clone()),
                                    )
                                    .when(team.description.is_some(), |d| {
                                        d.child(
                                            div()
                                                .text_sm()
                                                .text_color(theme.colors.text_muted)
                                                .child(team.description.clone().unwrap_or_default()),
                                        )
                                    })
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child(format!(
                                                "{} members · {} projects",
                                                team.active_member_count(),
                                                team.project_ids.len()
                                            )),
                                    ),
                            ),
                    ),
            )
            .child(
                // Tab navigation
                div()
                    .flex()
                    .gap_1()
                    .px_4()
                    .py_2()
                    .border_b_1()
                    .border_color(theme.colors.border)
                    .child(self.render_tab_button("Members", TeamViewMode::Members, cx))
                    .child(self.render_tab_button("Projects", TeamViewMode::Projects, cx)),
            )
            .child(
                // Tab content
                div()
                    .flex_1()
                    .id("scroll-team-details")
                    .overflow_y_scroll()
                    .child(match self.view_mode {
                        TeamViewMode::Members => self.render_members(team, cx).into_any_element(),
                        TeamViewMode::Projects => self.render_projects(team, cx).into_any_element(),
                        _ => div().into_any_element(),
                    }),
            )
    }
}
