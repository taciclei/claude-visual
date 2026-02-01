//! Logs section rendering

use gpui::*;
use gpui::prelude::*;
use super::super::types::*;

pub(crate) fn render_logs(
    logs: &[LogEntry],
    colors: &SimpleColors,
) -> impl IntoElement {
    // Copy colors for closures
    let colors_background = colors.background;
    let colors_text_muted = colors.text_muted;
    let colors_success = colors.success;
    let colors_warning = colors.warning;
    let colors_error = colors.error;
    let colors_text = colors.text;

    div()
        .max_h(px(200.0))
        .id("logs-scroll-container")
        .overflow_y_scroll()
        .px_3()
        .py_2()
        .bg(colors_background)
        .children(logs.iter().rev().take(20).map(|log| {
            let (icon, color) = match log.level {
                LogLevel::Info => ("i", colors_text_muted),
                LogLevel::Success => ("✓", colors_success),
                LogLevel::Warning => ("!", colors_warning),
                LogLevel::Error => ("✗", colors_error),
            };

            div()
                .flex()
                .items_center()
                .gap_2()
                .py_0p5()
                .child(
                    div()
                        .text_xs()
                        .text_color(color)
                        .w(px(16.0))
                        .text_center()
                        .child(icon),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(colors_text_muted)
                        .child(log.timestamp.format("%H:%M:%S").to_string()),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(colors_text)
                        .flex_1()
                        .child(log.message.clone()),
                )
        }))
}
