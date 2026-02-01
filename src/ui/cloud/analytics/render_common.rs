//! Common rendering utilities

use gpui::*;
use gpui::prelude::*;

use crate::cloud::team::AnalyticsPeriod;
use super::panel::AnalyticsPanel;
use super::types::AnalyticsViewMode;

impl AnalyticsPanel {
    /// Render period selector
    pub(super) fn render_period_selector(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .flex()
            .gap_1()
            .child(self.render_period_button(AnalyticsPeriod::Week, cx))
            .child(self.render_period_button(AnalyticsPeriod::Month, cx))
            .child(self.render_period_button(AnalyticsPeriod::Quarter, cx))
            .child(self.render_period_button(AnalyticsPeriod::Year, cx))
    }

    /// Render period button
    pub(super) fn render_period_button(&self, period: AnalyticsPeriod, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let is_selected = self.period == period;

        div()
            .id(ElementId::Name(format!("period-{:?}", period).into()))
            .px_2()
            .py_1()
            .rounded_md()
            .bg(if is_selected {
                theme.colors.accent
            } else {
                theme.colors.surface
            })
            .hover(|s| {
                s.bg(if is_selected {
                    theme.colors.accent_hover
                } else {
                    theme.colors.surface_hover
                })
            })
            .cursor_pointer()
            .text_xs()
            .text_color(theme.colors.text)
            .on_click(cx.listener(move |this, _, _window, cx| {
                this.change_period(period, cx);
            }))
            .child(period.display_name())
    }

    /// Render tab button
    pub(super) fn render_tab_button(&self, label: &str, mode: AnalyticsViewMode, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let is_active = self.view_mode == mode;
        let label = label.to_string();

        div()
            .id(ElementId::Name(format!("analytics-tab-{:?}", mode).into()))
            .px_3()
            .py_1()
            .rounded_md()
            .bg(if is_active {
                theme.colors.accent
            } else {
                theme.colors.surface
            })
            .hover(|s| {
                s.bg(if is_active {
                    theme.colors.accent_hover
                } else {
                    theme.colors.surface_hover
                })
            })
            .cursor_pointer()
            .text_sm()
            .text_color(theme.colors.text)
            .on_click(cx.listener(move |this, _, _window, cx| {
                this.set_view_mode(mode, cx);
            }))
            .child(label)
    }

    /// Render stat card
    pub(super) fn render_stat_card(
        &self,
        label: &str,
        value: &str,
        sublabel: Option<&str>,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let label = label.to_string();
        let value = value.to_string();
        let sublabel = sublabel.map(|s| s.to_string());

        div()
            .flex_1()
            .px_4()
            .py_3()
            .rounded_lg()
            .bg(theme.colors.surface)
            .border_1()
            .border_color(theme.colors.border)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child(label),
                    )
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.colors.text)
                            .mt_1()
                            .child(value),
                    )
                    .when_some(sublabel, |d, s| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .mt_1()
                                .child(s),
                        )
                    }),
            )
    }

    /// Render loading state
    pub(super) fn render_loading(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .flex()
            .items_center()
            .justify_center()
            .h_full()
            .child(
                div()
                    .text_sm()
                    .text_color(theme.colors.text_muted)
                    .child("Loading analytics..."),
            )
    }

    /// Render empty state
    pub(super) fn render_empty_state(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .h_full()
            .child(
                div()
                    .text_2xl()
                    .text_color(theme.colors.text_muted)
                    .child("📈"),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(theme.colors.text_muted)
                    .mt_2()
                    .child("No analytics data"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .mt_1()
                    .child("Start using the app to see usage statistics"),
            )
    }
}
