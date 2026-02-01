//! Overview rendering

use gpui::*;
use gpui::prelude::*;

use crate::cloud::team::UsageAnalytics;
use super::panel::AnalyticsPanel;

impl AnalyticsPanel {
    /// Render overview
    pub(super) fn render_overview(&self, analytics: &UsageAnalytics, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .flex()
            .flex_col()
            .gap_4()
            .px_4()
            .py_4()
            .child(
                // Stats row
                div()
                    .flex()
                    .gap_3()
                    .child(self.render_stat_card(
                        "Conversations",
                        &self.format_number(analytics.total_conversations),
                        Some(&format!("in {}", self.period.display_name())),
                        cx,
                    ))
                    .child(self.render_stat_card(
                        "Messages",
                        &self.format_number(analytics.total_messages),
                        None,
                        cx,
                    ))
                    .child(self.render_stat_card(
                        "Tokens Used",
                        &self.format_number(analytics.total_tokens),
                        None,
                        cx,
                    ))
                    .child(self.render_stat_card(
                        "Active Users",
                        &analytics.active_users.to_string(),
                        None,
                        cx,
                    )),
            )
            .child(
                // Chart placeholder
                div()
                    .w_full()
                    .h(px(200.0))
                    .rounded_lg()
                    .bg(theme.colors.surface)
                    .border_1()
                    .border_color(theme.colors.border)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_2xl()
                                    .text_color(theme.colors.text_muted)
                                    .child("📊"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.colors.text_muted)
                                    .child("Usage chart coming soon"),
                            ),
                    ),
            )
    }
}
