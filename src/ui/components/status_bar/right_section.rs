//! Right section of status bar - session stats, usage, health, latency, vim mode

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;
use super::types::StatusBarEvent;
use super::helpers::{format_duration, render_usage_bar, health_indicator, latency_indicator};
use super::status_bar::StatusBar;

/// Render right section of status bar
pub(crate) fn render_right_section(
    session_cost: f64,
    session_tokens: u64,
    usage_percent: u8,
    session_duration: u64,
    session_health: f32,
    response_latency_ms: Option<u64>,
    vim_mode: bool,
    theme: &Theme,
    cx: &mut Context<StatusBar>,
) -> impl IntoElement {
    let usage_bar = render_usage_bar(usage_percent);
    let duration_display = format_duration(session_duration);
    let tokens_k = session_tokens / 1000;

    div()
        .flex()
        .items_center()
        .gap_2()
        // Session cost and tokens
        .child(
            div()
                .flex()
                .items_center()
                .gap_1()
                .text_color(theme.colors.text_muted)
                .child("S:")
                .child(
                    div()
                        .text_color(theme.colors.warning)
                        .child(format!("${:.2}", session_cost))
                )
                .child(
                    div()
                        .text_color(theme.colors.text_muted)
                        .child(format!("{}k", tokens_k))
                )
        )
        // Usage bar
        .child(
            div()
                .text_color(theme.colors.accent)
                .child(usage_bar)
        )
        // Usage percentage and duration
        .child(
            div()
                .text_color(theme.colors.text_muted)
                .child(format!("{}% ({})", usage_percent, duration_display))
        )
        // Session health indicator
        .child({
            let (icon, _label) = health_indicator(session_health);
            div()
                .text_xs()
                .child(icon)
        })
        // Response latency indicator
        .when_some(latency_indicator(response_latency_ms), |d, (icon, text)| {
            d.child(
                div()
                    .flex()
                    .items_center()
                    .gap_0p5()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child(icon)
                    .child(text)
            )
        })
        // Vim mode indicator (clickable to toggle)
        .when(vim_mode, |this| {
            this.child(
                div()
                    .id("status-vim")
                    .px_1p5()
                    .py_0p5()
                    .rounded_sm()
                    .cursor_pointer()
                    .bg(theme.colors.success.opacity(0.2))
                    .text_color(theme.colors.success)
                    .hover(|s| s.bg(theme.colors.success.opacity(0.3)))
                    .on_click(cx.listener(|_this, _, _window, cx| {
                        cx.emit(StatusBarEvent::ToggleVimMode);
                    }))
                    .child("VIM")
            )
        })
}
