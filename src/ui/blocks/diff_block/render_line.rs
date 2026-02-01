//! Individual diff line rendering

use gpui::*;
use gpui::prelude::*;

/// Render a single diff line with line numbers and content
pub fn render_line(
    prefix: char,
    content: String,
    old_ln: Option<usize>,
    new_ln: Option<usize>,
    line_type: &str,
    border_color: Hsla,
    surface_color: Hsla,
    text_color: Hsla,
    text_muted_color: Hsla,
    accent_color: Hsla,
    success_color: Hsla,
    error_color: Hsla,
    background_color: Hsla,
) -> impl IntoElement {
    let (bg_color, text_color, prefix_color) = match line_type {
        "added" => (
            success_color.opacity(0.12),
            text_color,
            success_color,
        ),
        "removed" => (
            error_color.opacity(0.12),
            text_color,
            error_color,
        ),
        "header" => (
            surface_color,
            accent_color,
            accent_color,
        ),
        _ => (
            background_color,
            text_color,
            text_muted_color,
        ),
    };

    let old_ln_str = old_ln.map(|n| format!("{:>4}", n)).unwrap_or_else(|| "    ".to_string());
    let new_ln_str = new_ln.map(|n| format!("{:>4}", n)).unwrap_or_else(|| "    ".to_string());

    div()
        .w_full()
        .min_w(px(600.0))
        .bg(bg_color)
        .flex()
        .flex_row()
        // Old line number
        .when(line_type != "header", |d| {
            d.child(
                div()
                    .w(px(40.0))
                    .flex_shrink_0()
                    .px_1()
                    .py_0p5()
                    .text_xs()
                    .font_family("JetBrains Mono")
                    .text_color(text_muted_color)
                    .text_right()
                    .border_r_1()
                    .border_color(border_color)
                    .child(old_ln_str),
            )
        })
        // New line number
        .when(line_type != "header", |d| {
            d.child(
                div()
                    .w(px(40.0))
                    .flex_shrink_0()
                    .px_1()
                    .py_0p5()
                    .text_xs()
                    .font_family("JetBrains Mono")
                    .text_color(text_muted_color)
                    .text_right()
                    .border_r_1()
                    .border_color(border_color)
                    .child(new_ln_str),
            )
        })
        // Prefix (+/-)
        .when(line_type != "header", |d| {
            let prefix_str = if line_type == "context" { " ".to_string() } else { prefix.to_string() };
            d.child(
                div()
                    .w(px(20.0))
                    .flex_shrink_0()
                    .px_1()
                    .py_0p5()
                    .text_xs()
                    .font_family("JetBrains Mono")
                    .text_color(prefix_color)
                    .child(prefix_str),
            )
        })
        // Content
        .child(
            div()
                .flex_1()
                .px_2()
                .py_0p5()
                .text_xs()
                .font_family("JetBrains Mono")
                .whitespace_nowrap()
                .text_color(text_color)
                .when(line_type == "header", |d| {
                    d.font_weight(FontWeight::MEDIUM)
                        .py_1()
                        .bg(surface_color)
                })
                .child(content),
        )
}
