//! Projects view rendering

use gpui::*;
use gpui::prelude::*;

use crate::cloud::team::UsageAnalytics;
use super::panel::AnalyticsPanel;

impl AnalyticsPanel {
    /// Render projects view
    pub(super) fn render_projects(&self, analytics: &UsageAnalytics, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let mut projects: Vec<_> = analytics.usage_by_project.values().collect();
        projects.sort_by(|a, b| b.tokens.cmp(&a.tokens));

        div()
            .flex()
            .flex_col()
            .px_4()
            .py_4()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.colors.text)
                    .mb_3()
                    .child("Usage by Project"),
            )
            .child(
                // Table header
                div()
                    .flex()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(theme.colors.border)
                    .child(
                        div()
                            .flex_1()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child("Project"),
                    )
                    .child(
                        div()
                            .w(px(80.0))
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .text_right()
                            .child("Contributors"),
                    )
                    .child(
                        div()
                            .w(px(80.0))
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .text_right()
                            .child("Messages"),
                    )
                    .child(
                        div()
                            .w(px(80.0))
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .text_right()
                            .child("Tokens"),
                    ),
            )
            .children(projects.into_iter().map(|project| {
                div()
                    .flex()
                    .items_center()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(theme.colors.border.opacity(0.5))
                    .hover(|s| s.bg(theme.colors.surface_hover))
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_lg()
                                    .child("📁"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.colors.text)
                                    .child(project.project_name.clone()),
                            ),
                    )
                    .child(
                        div()
                            .w(px(80.0))
                            .text_sm()
                            .text_color(theme.colors.text)
                            .text_right()
                            .child(project.contributors.to_string()),
                    )
                    .child(
                        div()
                            .w(px(80.0))
                            .text_sm()
                            .text_color(theme.colors.text)
                            .text_right()
                            .child(self.format_number(project.messages)),
                    )
                    .child(
                        div()
                            .w(px(80.0))
                            .text_sm()
                            .text_color(theme.colors.text)
                            .text_right()
                            .child(self.format_number(project.tokens)),
                    )
            }))
            .when(analytics.usage_by_project.is_empty(), |d| {
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .py_8()
                        .text_sm()
                        .text_color(theme.colors.text_muted)
                        .child("No project data available"),
                )
            })
    }
}
