//! Latency indicator rendering for ChatView

use gpui::*;
use gpui::prelude::*;
use crate::ui::pct;
use super::super::super::core::ChatView;

impl ChatView {
    pub fn render_latency_indicator(&self, theme: &crate::app::theme::Theme) -> Div {
        let latency = self.stats.last_response_latency_ms.unwrap_or(0);
        let avg = self.stats.avg_response_latency_ms as u64;
        let (latency_color, latency_icon) = if latency < 2000 {
            (theme.colors.success, "⚡")
        } else if latency < 5000 {
            (theme.colors.warning, "⏱")
        } else {
            (theme.colors.error, "🐌")
        };

        div()
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py_px()
            .rounded_sm()
            .bg(latency_color.opacity(0.1))
            .text_xs()
            .text_color(latency_color)
            .child(latency_icon)
            .child(if latency > 0 {
                format!("{}ms", latency)
            } else {
                "—".to_string()
            })
            .when(avg > 0.0 as u64, |d| {
                d.child(
                    div()
                        .text_xs()
                        .text_color(theme.colors.text_muted.opacity(0.5))
                        .child(format!("(avg {}ms)", avg))
                )
            })
    }
}
