//! MCP panel header rendering

use gpui::prelude::*;
use gpui::*;

use crate::app::theme::Theme;

pub(crate) fn render_header<F>(
    theme: &Theme,
    connected_count: usize,
    total_servers: usize,
    total_tools: usize,
    total_resources: usize,
    on_close: F,
) -> impl IntoElement
where
    F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    div()
        .px_4()
        .py_3()
        .border_b_1()
        .border_color(theme.colors.border)
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(div().text_base().child("🔌"))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.colors.text)
                        .child("MCP Servers"),
                )
                .child(
                    div()
                        .px_2()
                        .py_px()
                        .rounded_full()
                        .bg(if connected_count == total_servers && total_servers > 0 {
                            theme.colors.success.opacity(0.2)
                        } else if connected_count > 0 {
                            theme.colors.warning.opacity(0.2)
                        } else {
                            theme.colors.text_muted.opacity(0.2)
                        })
                        .text_xs()
                        .text_color(if connected_count == total_servers && total_servers > 0 {
                            theme.colors.success
                        } else if connected_count > 0 {
                            theme.colors.warning
                        } else {
                            theme.colors.text_muted
                        })
                        .child(format!("{}/{} connected", connected_count, total_servers)),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                // Stats badges
                .when(total_tools > 0, |d| {
                    let info_color = theme.colors.info;
                    d.child(
                        div()
                            .px_2()
                            .py_px()
                            .rounded_full()
                            .bg(info_color.opacity(0.1))
                            .text_xs()
                            .text_color(info_color)
                            .child(format!("🔧 {}", total_tools)),
                    )
                })
                .when(total_resources > 0, |d| {
                    let accent_color = theme.colors.accent;
                    d.child(
                        div()
                            .px_2()
                            .py_px()
                            .rounded_full()
                            .bg(accent_color.opacity(0.1))
                            .text_xs()
                            .text_color(accent_color)
                            .child(format!("📄 {}", total_resources)),
                    )
                })
                // Close button
                .child(
                    div()
                        .id("close-mcp-panel")
                        .px_2()
                        .py_1()
                        .rounded_md()
                        .cursor_pointer()
                        .text_sm()
                        .text_color(theme.colors.text_muted)
                        .hover({
                            let surface_hover = theme.colors.surface_hover;
                            move |s| s.bg(surface_hover)
                        })
                        .on_click(on_close)
                        .child("×"),
                ),
        )
}
