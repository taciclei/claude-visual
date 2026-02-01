//! Users view rendering

use gpui::prelude::*;
use gpui::*;

use super::panel::AnalyticsPanel;
use crate::cloud::team::UsageAnalytics;

impl AnalyticsPanel {
    /// Render users view
    pub(super) fn render_users(
        &self,
        analytics: &UsageAnalytics,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let mut users: Vec<_> = analytics.usage_by_user.values().collect();
        users.sort_by(|a, b| b.tokens.cmp(&a.tokens));

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
                    .child("Usage by User"),
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
                            .child("User"),
                    )
                    .child(
                        div()
                            .w(px(80.0))
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .text_right()
                            .child("Conversations"),
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
            .children(users.into_iter().map(|user| {
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
                                    .size_6()
                                    .rounded_full()
                                    .bg(theme.colors.accent)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_xs()
                                    .text_color(theme.colors.text)
                                    .child(
                                        user.user_name
                                            .as_ref()
                                            .and_then(|n| n.chars().next())
                                            .unwrap_or('?')
                                            .to_string(),
                                    ),
                            )
                            .child(
                                div().text_sm().text_color(theme.colors.text).child(
                                    user.user_name
                                        .clone()
                                        .unwrap_or_else(|| user.user_id.clone()),
                                ),
                            ),
                    )
                    .child(
                        div()
                            .w(px(80.0))
                            .text_sm()
                            .text_color(theme.colors.text)
                            .text_right()
                            .child(self.format_number(user.conversations)),
                    )
                    .child(
                        div()
                            .w(px(80.0))
                            .text_sm()
                            .text_color(theme.colors.text)
                            .text_right()
                            .child(self.format_number(user.messages)),
                    )
                    .child(
                        div()
                            .w(px(80.0))
                            .text_sm()
                            .text_color(theme.colors.text)
                            .text_right()
                            .child(self.format_number(user.tokens)),
                    )
            }))
            .when(analytics.usage_by_user.is_empty(), |d| {
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .py_8()
                        .text_sm()
                        .text_color(theme.colors.text_muted)
                        .child("No user data available"),
                )
            })
    }
}
