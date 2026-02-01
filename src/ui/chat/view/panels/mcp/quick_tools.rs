//! Quick MCP tools dropdown for fast tool access

use gpui::*;
use gpui::prelude::*;

use super::super::super::core::ChatView;

impl ChatView {
    /// Render quick MCP tools dropdown (shown on hover/click of MCP button)
    pub(crate) fn render_mcp_quick_tools(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let quick_tools = self.get_quick_mcp_tools();
        let all_tools = self.get_all_mcp_tools();
        let has_tools = !all_tools.is_empty();
        let has_quick_tools = !quick_tools.is_empty();

        let surface = theme.colors.surface;
        let surface_hover = theme.colors.surface_hover;
        let text = theme.colors.text;
        let text_muted = theme.colors.text_muted;
        let accent = theme.colors.accent;
        let warning = theme.colors.warning;
        let info = theme.colors.info;
        let border = theme.colors.border;

        // Close dropdown when clicking outside
        let overlay_click = cx.listener(|this, _, _window, cx| {
            this.panels.mcp_quick_tools = false;
            cx.notify();
        });

        div()
            .id("mcp-quick-tools-overlay")
            .absolute()
            .inset_0()
            .on_click(overlay_click)
            .child(
                div()
                    .id("mcp-quick-tools-dropdown")
                    .absolute()
                    .bottom(px(50.0))
                    .right(px(16.0))
                    .w(px(280.0))
                    .max_h(px(350.0))
                    .bg(surface)
                    .rounded_lg()
                    .border_1()
                    .border_color(border)
                    .shadow_lg()
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    .on_click(|_, _, _| {}) // Prevent closing when clicking inside
                    // Header
                    .child(
                        div()
                            .px_3()
                            .py_2()
                            .border_b_1()
                            .border_color(border)
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .child("🔧")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(text)
                                            .child("Quick Tools")
                                    )
                            )
                            .child(
                                div()
                                    .id("mcp-quick-expand")
                                    .px_2()
                                    .py_1()
                                    .rounded_sm()
                                    .text_xs()
                                    .text_color(accent)
                                    .hover(move |s| s.bg(accent.opacity(0.1)))
                                    .cursor_pointer()
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.panels.mcp_quick_tools = false;
                                        this.panels.mcp_panel = true;
                                        cx.notify();
                                    }))
                                    .child("View all →")
                            )
                    )
                    // Content
                    .child(
                        div()
                            .id("mcp-quick-tools-list")
                            .flex_1()
                            .overflow_y_scroll()
                            .when(!has_tools, |d| {
                                d.child(
                                    div()
                                        .px_3()
                                        .py_6()
                                        .flex()
                                        .flex_col()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .text_2xl()
                                                .child("🔌")
                                        )
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(text_muted)
                                                .text_center()
                                                .child("No MCP tools available")
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(text_muted)
                                                .text_center()
                                                .child("Connect to MCP servers in settings")
                                        )
                                )
                            })
                            .when(has_tools && has_quick_tools, |d| {
                                d.child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        // Favorite/Recent tools section
                                        .children(quick_tools.iter().enumerate().map(|(idx, (tool_name, is_favorite))| {
                                            let tool_name_click = tool_name.clone();
                                            let tool_name_fav = tool_name.clone();
                                            let is_fav = *is_favorite;

                                            div()
                                                .id(ElementId::Name(format!("quick-tool-{}", idx).into()))
                                                .px_3()
                                                .py_2()
                                                .flex()
                                                .items_center()
                                                .justify_between()
                                                .hover(move |s| s.bg(surface_hover))
                                                .cursor_pointer()
                                                .on_click(cx.listener(move |this, _, _window, cx| {
                                                    this.use_mcp_tool(&tool_name_click, cx);
                                                }))
                                                .child(
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap_2()
                                                        .flex_1()
                                                        .overflow_hidden()
                                                        .child(
                                                            div()
                                                                .text_sm()
                                                                .text_color(info)
                                                                .child("🔧")
                                                        )
                                                        .child(
                                                            div()
                                                                .text_sm()
                                                                .text_color(text)
                                                                .truncate()
                                                                .child(format_tool_name(tool_name))
                                                        )
                                                )
                                                .child(
                                                    div()
                                                        .id(ElementId::Name(format!("quick-tool-fav-{}", idx).into()))
                                                        .px_1()
                                                        .py_0p5()
                                                        .rounded_sm()
                                                        .text_sm()
                                                        .text_color(if is_fav { warning } else { text_muted })
                                                        .hover(move |s| s.bg(warning.opacity(0.1)).text_color(warning))
                                                        .cursor_pointer()
                                                        .on_click(cx.listener(move |this, _, _window, cx| {
                                                            this.toggle_mcp_tool_favorite(&tool_name_fav, cx);
                                                        }))
                                                        .child(if is_fav { "★" } else { "☆" })
                                                )
                                        }))
                                )
                            })
                            .when(has_tools && !has_quick_tools, |d| {
                                d.child(
                                    div()
                                        .px_3()
                                        .py_4()
                                        .flex()
                                        .flex_col()
                                        .gap_2()
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(text_muted)
                                                .child("No recent tools yet")
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(text_muted)
                                                .child("Use tools to see them here")
                                        )
                                )
                            })
                    )
                    // Footer with tool count
                    .when(has_tools, |d| {
                        d.child(
                            div()
                                .px_3()
                                .py_2()
                                .border_t_1()
                                .border_color(border)
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(text_muted)
                                        .child(format!("{} tools available", all_tools.len()))
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(text_muted)
                                        .child("★ to favorite")
                                )
                        )
                    })
            )
    }
}

/// Format tool name for display (removes mcp__ prefix and shows cleaner name)
fn format_tool_name(name: &str) -> String {
    // Remove mcp__server__ prefix if present
    let name = if name.starts_with("mcp__") {
        let parts: Vec<&str> = name.split("__").collect();
        if parts.len() >= 3 {
            // mcp__server__tool -> tool
            parts[2..].join("__")
        } else {
            name.to_string()
        }
    } else if name.contains(":") {
        // server:tool -> tool
        name.split(':').last().unwrap_or(name).to_string()
    } else {
        name.to_string()
    };

    // Convert snake_case to Title Case
    name.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
