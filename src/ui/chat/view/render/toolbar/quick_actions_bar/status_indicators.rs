//! Status indicators rendering (context usage and response time)

use gpui::*;
use gpui::prelude::*;
use crate::ui::pct;
use crate::ui::chat::view::core::ChatView;
use crate::ui::chat::view::types::ChatViewEvent;

impl ChatView {
    pub(super) fn render_status_indicators(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let usage_pct = self.context_usage_percentage();
        let needs_compact = usage_pct > 0.7;
        let critical = usage_pct > 0.9;

        div()
            .flex()
            .items_center()
            .gap_2()
            // Context-aware action suggestion
            .when(needs_compact, |d| {
                let (icon, label, cmd, color) = if critical {
                    ("🗜️", "Compact Now!", "/compact", theme.colors.error)
                } else {
                    ("🗜️", "Compact", "/compact", theme.colors.warning)
                };
                // Pre-capture for closure
                let hover_bg = color.opacity(0.25);
                d.child(
                    div()
                        .id("ctx-compact-suggestion")
                        .flex()
                        .items_center()
                        .gap_1()
                        .px_2()
                        .py(px(2.0))
                        .rounded_md()
                        .cursor_pointer()
                        .bg(color.opacity(0.15))
                        .border_1()
                        .border_color(color.opacity(0.3))
                        .text_xs()
                        .text_color(color)
                        .hover(move |s| s.bg(hover_bg))
                        .on_click(cx.listener(move |_this, _, _window, cx| {
                            cx.emit(ChatViewEvent::Submit(cmd.to_string()));
                        }))
                        .child(icon)
                        .child(label)
                )
            })
            // Context usage mini-indicator (compact bar)
            .when(self.context_used > 0, |d| {
                let usage_color = if usage_pct < 0.5 {
                    theme.colors.success
                } else if usage_pct < 0.8 {
                    theme.colors.warning
                } else {
                    theme.colors.error
                };
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .ml_auto()
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted.opacity(0.5))
                                .child("ctx")
                        )
                        .child(
                            // Mini progress bar
                            div()
                                .w(px(40.0))
                                .h(px(4.0))
                                .rounded_sm()
                                .bg(theme.colors.surface_hover)
                                .child(
                                    div()
                                        .h_full()
                                        .rounded_sm()
                                        .bg(usage_color)
                                        .w(pct((usage_pct * 100.0) as f32))
                                )
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(usage_color.opacity(0.8))
                                .child(format!("{:.0}%", usage_pct * 100.0))
                        )
                )
            })
            // Last response time (if available)
            .when_some(self.last_response_time_ms, |d, time_ms| {
                let (time_str, time_color) = if time_ms < 1000 {
                    (format!("{}ms", time_ms), theme.colors.success)
                } else if time_ms < 5000 {
                    (format!("{:.1}s", time_ms as f64 / 1000.0), theme.colors.warning)
                } else {
                    (format!("{:.1}s", time_ms as f64 / 1000.0), theme.colors.error)
                };
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .ml_auto()
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted.opacity(0.5))
                                .child("⏱")
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(time_color.opacity(0.8))
                                .child(time_str)
                        )
                )
            })
    }
}
