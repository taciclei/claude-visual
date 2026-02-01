//! Analytics panel component

use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;

use super::types::{AnalyticsPanelEvent, AnalyticsViewMode};
use crate::app::state::AppState;
use crate::cloud::team::{AnalyticsPeriod, UsageAnalytics};

impl EventEmitter<AnalyticsPanelEvent> for AnalyticsPanel {}

/// Analytics dashboard panel
pub struct AnalyticsPanel {
    pub(super) app_state: Arc<AppState>,
    /// Team ID
    pub(super) team_id: Option<String>,
    /// Team name
    pub(super) team_name: Option<String>,
    /// Current analytics data
    pub(super) analytics: Option<UsageAnalytics>,
    /// Selected period
    pub(super) period: AnalyticsPeriod,
    /// Is loading
    pub(super) is_loading: bool,
    /// Error message
    pub(super) error_message: Option<String>,
    /// View mode
    pub(super) view_mode: AnalyticsViewMode,
    /// Focus handle
    pub(super) focus_handle: FocusHandle,
}

impl AnalyticsPanel {
    /// Create a new analytics panel
    pub fn new(app_state: Arc<AppState>, cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            team_id: None,
            team_name: None,
            analytics: None,
            period: AnalyticsPeriod::Month,
            is_loading: false,
            error_message: None,
            view_mode: AnalyticsViewMode::Overview,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Set team
    pub fn set_team(&mut self, team_id: String, team_name: String, cx: &mut Context<Self>) {
        self.team_id = Some(team_id);
        self.team_name = Some(team_name);
        cx.notify();
    }

    /// Set analytics data
    pub fn set_analytics(&mut self, analytics: UsageAnalytics, cx: &mut Context<Self>) {
        self.analytics = Some(analytics);
        self.is_loading = false;
        cx.notify();
    }

    /// Set loading state
    pub fn set_loading(&mut self, loading: bool, cx: &mut Context<Self>) {
        self.is_loading = loading;
        cx.notify();
    }

    /// Set error
    pub fn set_error(&mut self, error: Option<String>, cx: &mut Context<Self>) {
        self.error_message = error;
        self.is_loading = false;
        cx.notify();
    }

    /// Change period
    pub(super) fn change_period(&mut self, period: AnalyticsPeriod, cx: &mut Context<Self>) {
        self.period = period;
        cx.emit(AnalyticsPanelEvent::ChangePeriod(period));
        cx.notify();
    }

    /// Set view mode
    pub(super) fn set_view_mode(&mut self, mode: AnalyticsViewMode, cx: &mut Context<Self>) {
        self.view_mode = mode;
        cx.notify();
    }

    /// Format large number
    pub(super) fn format_number(&self, n: u64) -> String {
        if n >= 1_000_000 {
            format!("{:.1}M", n as f64 / 1_000_000.0)
        } else if n >= 1_000 {
            format!("{:.1}K", n as f64 / 1_000.0)
        } else {
            n.to_string()
        }
    }
}

impl Render for AnalyticsPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.colors.background)
            .child(
                // Header
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .py_3()
                    .border_b_1()
                    .border_color(theme.colors.border)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.colors.text)
                                    .child("Usage Analytics"),
                            )
                            .when(self.team_name.is_some(), |d| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .child(self.team_name.clone().unwrap_or_default()),
                                )
                            }),
                    )
                    .child(self.render_period_selector(cx)),
            )
            // View tabs
            .child(
                div()
                    .flex()
                    .gap_1()
                    .px_4()
                    .py_2()
                    .border_b_1()
                    .border_color(theme.colors.border)
                    .child(self.render_tab_button("Overview", AnalyticsViewMode::Overview, cx))
                    .child(self.render_tab_button("Users", AnalyticsViewMode::Users, cx))
                    .child(self.render_tab_button("Projects", AnalyticsViewMode::Projects, cx))
                    .child(self.render_tab_button("Timeline", AnalyticsViewMode::Timeline, cx)),
            )
            // Content
            .child(
                div()
                    .flex_1()
                    .id("scroll-analytics-content")
                    .overflow_y_scroll()
                    .child(if self.is_loading {
                        self.render_loading(cx).into_any_element()
                    } else if let Some(ref analytics) = self.analytics {
                        match self.view_mode {
                            AnalyticsViewMode::Overview => {
                                self.render_overview(analytics, cx).into_any_element()
                            }
                            AnalyticsViewMode::Users => {
                                self.render_users(analytics, cx).into_any_element()
                            }
                            AnalyticsViewMode::Projects => {
                                self.render_projects(analytics, cx).into_any_element()
                            }
                            AnalyticsViewMode::Timeline => {
                                self.render_timeline(analytics, cx).into_any_element()
                            }
                        }
                    } else {
                        self.render_empty_state(cx).into_any_element()
                    }),
            )
    }
}
