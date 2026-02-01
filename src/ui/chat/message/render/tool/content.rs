//! Tool content rendering

use gpui::prelude::*;
use gpui::*;

use super::types::ToolDisplay;
use crate::app::theme::Theme;

/// Render the content element based on tool display type
pub(super) fn render_content(tool_display: &ToolDisplay, theme: &Theme, is_error: bool) -> Div {
    match tool_display {
        ToolDisplay::FilePath { display, .. } => div()
            .text_xs()
            .font_family("JetBrains Mono")
            .text_color(theme.colors.text_muted)
            .child(display.clone()),
        ToolDisplay::Command { display, desc, .. } => div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .child(
                div()
                    .text_xs()
                    .font_family("JetBrains Mono")
                    .text_color(theme.colors.text)
                    .bg(theme.colors.surface)
                    .px_2()
                    .py_1()
                    .rounded_sm()
                    .child(display.clone()),
            )
            .when(!desc.is_empty(), |d| {
                d.child(
                    div()
                        .text_xs()
                        .text_color(theme.colors.text_muted)
                        .child(desc.clone()),
                )
            }),
        ToolDisplay::Edit {
            file_path,
            old_text,
            new_text,
        } => render_edit_content(file_path, old_text, new_text, theme),
        ToolDisplay::Pattern { display, path, .. } => div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .child(
                div()
                    .text_xs()
                    .font_family("JetBrains Mono")
                    .text_color(theme.colors.info)
                    .child(display.clone()),
            )
            .when_some(path.as_ref(), |d, p| {
                d.child(
                    div()
                        .text_xs()
                        .text_color(theme.colors.text_muted)
                        .child(format!("in {}", p)),
                )
            }),
        ToolDisplay::Prompt { display } => div()
            .text_xs()
            .text_color(theme.colors.text_muted)
            .child(display.clone()),
        ToolDisplay::Json(s) => {
            let line_count = s.lines().count();
            let truncated = if line_count > 10 {
                let lines: Vec<&str> = s.lines().take(10).collect();
                format!("{}...\n({} more lines)", lines.join("\n"), line_count - 10)
            } else {
                s.clone()
            };
            div()
                .text_xs()
                .font_family("JetBrains Mono")
                .text_color(theme.colors.text_muted)
                .max_h(px(150.0))
                .overflow_hidden()
                .child(truncated)
        }
        ToolDisplay::Plain(s) => {
            let line_count = s.lines().count();
            let truncated = if line_count > 15 {
                let lines: Vec<&str> = s.lines().take(15).collect();
                format!("{}...\n({} more lines)", lines.join("\n"), line_count - 15)
            } else if s.chars().count() > 1000 {
                format!(
                    "{}... ({} chars)",
                    s.chars().take(1000).collect::<String>(),
                    s.len()
                )
            } else {
                s.clone()
            };
            div()
                .text_xs()
                .font_family("JetBrains Mono")
                .text_color(if is_error {
                    theme.colors.error
                } else {
                    theme.colors.text_muted
                })
                .max_h(px(200.0))
                .overflow_hidden()
                .child(truncated)
        }
    }
}

/// Render edit operation content with old/new text
fn render_edit_content(
    file_path: &str,
    old_text: &Option<String>,
    new_text: &Option<String>,
    theme: &Theme,
) -> Div {
    // Copy theme colors for move closures
    let error_color = theme.colors.error;
    let success_color = theme.colors.success;

    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .text_xs()
                .font_family("JetBrains Mono")
                .text_color(theme.colors.text_muted)
                .child(format!("📄 {}", file_path)),
        )
        .when_some(old_text.as_ref(), move |d, old| {
            let truncated = if old.chars().count() > 100 {
                format!("{}...", old.chars().take(100).collect::<String>())
            } else {
                old.clone()
            };
            d.child(
                div()
                    .flex()
                    .items_start()
                    .gap_2()
                    .child(div().text_xs().text_color(error_color).child("−"))
                    .child(
                        div()
                            .text_xs()
                            .font_family("JetBrains Mono")
                            .text_color(error_color.opacity(0.8))
                            .bg(error_color.opacity(0.1))
                            .px_1()
                            .rounded_sm()
                            .overflow_hidden()
                            .child(truncated),
                    ),
            )
        })
        .when_some(new_text.as_ref(), move |d, new| {
            let truncated = if new.chars().count() > 100 {
                format!("{}...", new.chars().take(100).collect::<String>())
            } else {
                new.clone()
            };
            d.child(
                div()
                    .flex()
                    .items_start()
                    .gap_2()
                    .child(div().text_xs().text_color(success_color).child("+"))
                    .child(
                        div()
                            .text_xs()
                            .font_family("JetBrains Mono")
                            .text_color(success_color.opacity(0.8))
                            .bg(success_color.opacity(0.1))
                            .px_1()
                            .rounded_sm()
                            .overflow_hidden()
                            .child(truncated),
                    ),
            )
        })
}
