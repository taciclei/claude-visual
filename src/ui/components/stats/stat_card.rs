//! StatCard component

use gpui::prelude::*;
use gpui::*;
use std::sync::Arc;

use super::types::*;
use crate::app::state::AppState;

/// Stat card component
pub struct StatCard {
    app_state: Arc<AppState>,
    /// Stat label
    label: String,
    /// Stat value
    value: String,
    /// Optional icon
    icon: Option<String>,
    /// Size variant
    size: StatSize,
    /// Trend direction
    trend: TrendDirection,
    /// Trend value (e.g., "+12%")
    trend_value: Option<String>,
    /// Whether trend up is good
    trend_up_is_good: bool,
    /// Optional subtitle/description
    subtitle: Option<String>,
    /// Show in bordered card
    bordered: bool,
}

impl StatCard {
    pub fn new(
        app_state: Arc<AppState>,
        label: impl Into<String>,
        value: impl Into<String>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            app_state,
            label: label.into(),
            value: value.into(),
            icon: None,
            size: StatSize::default(),
            trend: TrendDirection::default(),
            trend_value: None,
            trend_up_is_good: true,
            subtitle: None,
            bordered: true,
        }
    }

    /// Set value
    pub fn set_value(&mut self, value: impl Into<String>, cx: &mut Context<Self>) {
        self.value = value.into();
        cx.notify();
    }

    /// Set label
    pub fn set_label(&mut self, label: impl Into<String>, cx: &mut Context<Self>) {
        self.label = label.into();
        cx.notify();
    }

    /// Set icon
    pub fn set_icon(&mut self, icon: Option<String>, cx: &mut Context<Self>) {
        self.icon = icon;
        cx.notify();
    }

    /// Set size
    pub fn set_size(&mut self, size: StatSize, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }

    /// Set trend
    pub fn set_trend(
        &mut self,
        direction: TrendDirection,
        value: Option<String>,
        cx: &mut Context<Self>,
    ) {
        self.trend = direction;
        self.trend_value = value;
        cx.notify();
    }

    /// Set whether trend up is good
    pub fn set_trend_up_is_good(&mut self, is_good: bool, cx: &mut Context<Self>) {
        self.trend_up_is_good = is_good;
        cx.notify();
    }

    /// Set subtitle
    pub fn set_subtitle(&mut self, subtitle: Option<String>, cx: &mut Context<Self>) {
        self.subtitle = subtitle;
        cx.notify();
    }

    /// Set bordered
    pub fn set_bordered(&mut self, bordered: bool, cx: &mut Context<Self>) {
        self.bordered = bordered;
        cx.notify();
    }
}

impl Render for StatCard {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let value_size = self.size.value_size();
        let label_size = self.size.label_size();

        let success = hsla(0.38, 0.7, 0.45, 1.0);
        let error = hsla(0.0, 0.7, 0.5, 1.0);

        let trend_color = match self.trend {
            TrendDirection::Neutral => theme.colors.text_muted,
            TrendDirection::Up => {
                if self.trend_up_is_good {
                    success
                } else {
                    error
                }
            }
            TrendDirection::Down => {
                if self.trend_up_is_good {
                    error
                } else {
                    success
                }
            }
        };

        div()
            .id("stat-card")
            .p_4()
            .when(self.bordered, |d| {
                d.rounded(px(8.0))
                    .border_1()
                    .border_color(theme.colors.border)
                    .bg(theme.colors.surface)
            })
            .flex()
            .flex_col()
            .gap_2()
            // Header with icon
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    // Label
                    .child(
                        div()
                            .text_size(px(label_size))
                            .text_color(theme.colors.text_muted)
                            .font_weight(FontWeight::MEDIUM)
                            .child(self.label.clone()),
                    )
                    // Icon
                    .when_some(self.icon.clone(), |d, icon| {
                        d.child(
                            div()
                                .text_base()
                                .text_color(theme.colors.text_muted)
                                .child(icon),
                        )
                    }),
            )
            // Value
            .child(
                div()
                    .text_size(px(value_size))
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.colors.text)
                    .child(self.value.clone()),
            )
            // Trend
            .when_some(
                if self.trend != TrendDirection::Neutral || self.trend_value.is_some() {
                    Some((self.trend.icon().to_string(), self.trend_value.clone()))
                } else {
                    None
                },
                |d, (trend_icon, trend_value)| {
                    d.child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(trend_color)
                                    .child(trend_icon.clone()),
                            )
                            .when_some(trend_value, |d, value| {
                                d.child(div().text_sm().text_color(trend_color).child(value))
                            }),
                    )
                },
            )
            // Subtitle
            .when_some(self.subtitle.clone(), |d, subtitle| {
                d.child(
                    div()
                        .text_xs()
                        .text_color(theme.colors.text_muted)
                        .child(subtitle),
                )
            })
    }
}
