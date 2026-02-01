//! Timeline view rendering

use gpui::prelude::*;
use gpui::*;

use super::panel::AnalyticsPanel;
use crate::cloud::team::UsageAnalytics;

impl AnalyticsPanel {
    /// Render timeline view
    pub(super) fn render_timeline(
        &self,
        analytics: &UsageAnalytics,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

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
                    .child("Daily Breakdown"),
            )
            .children(analytics.daily_breakdown.iter().map(|day| {
                div()
                    .flex()
                    .items_center()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(theme.colors.border.opacity(0.5))
                    .child(
                        div()
                            .w(px(100.0))
                            .text_sm()
                            .text_color(theme.colors.text_muted)
                            .child(day.date.format("%b %d").to_string()),
                    )
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .gap_4()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.colors.text)
                                    .child(format!("{} conversations", day.conversations)),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.colors.text)
                                    .child(format!("{} messages", day.messages)),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.colors.text)
                                    .child(format!("{} tokens", self.format_number(day.tokens))),
                            ),
                    )
            }))
            .when(analytics.daily_breakdown.is_empty(), |d| {
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .py_8()
                        .text_sm()
                        .text_color(theme.colors.text_muted)
                        .child("No timeline data available"),
                )
            })
    }
}
