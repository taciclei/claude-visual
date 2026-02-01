//! Individual permission item component

use gpui::*;
use gpui::prelude::*;

use super::super::super::core::ChatView;
use super::super::super::types::PermissionRequest;

/// Get a human-readable description for a Claude Code tool
fn get_tool_description(tool: &str) -> &'static str {
    match tool.to_lowercase().as_str() {
        "bash" | "execute" | "shell" => "Execute terminal command",
        "read" => "Read file contents",
        "write" => "Write/create file",
        "edit" => "Modify existing file",
        "glob" => "Search for files by pattern",
        "grep" => "Search file contents",
        "webfetch" | "fetch" => "Fetch web content",
        "websearch" => "Search the web",
        "notebookedit" => "Edit Jupyter notebook",
        "task" => "Launch subagent task",
        "mcp" => "Use MCP server tool",
        _ => "Execute tool action",
    }
}

/// Get icon for a Claude Code tool
fn get_tool_icon(tool: &str) -> &'static str {
    match tool.to_lowercase().as_str() {
        "bash" | "execute" | "shell" => "💻",
        "read" => "📖",
        "write" => "✏️",
        "edit" => "📝",
        "glob" => "🔍",
        "grep" => "🔎",
        "webfetch" | "fetch" => "🌐",
        "websearch" => "🔍",
        "notebookedit" => "📓",
        "task" => "🤖",
        "mcp" => "🔌",
        _ => "⚙️",
    }
}

pub fn render_permissions_list(
    theme: &crate::app::theme::Theme,
    permissions: Vec<(usize, &PermissionRequest)>,
    cx: &mut Context<ChatView>,
) -> impl IntoElement {
    div()
        .id("permissions-list")
        .flex_1()
        .overflow_y_scroll()
        .when(permissions.is_empty(), |d| {
            d.child(
                div()
                    .px_4()
                    .py_8()
                    .text_center()
                    .child(
                        div()
                            .text_lg()
                            .mb_2()
                            .child("✅")
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text_muted)
                            .child("No pending permissions")
                    )
            )
        })
        .children(permissions.into_iter().map(|(idx, permission)| {
            render_permission_item(theme, idx, permission, cx)
        }))
}

fn render_permission_item(
    theme: &crate::app::theme::Theme,
    idx: usize,
    permission: &PermissionRequest,
    cx: &mut Context<ChatView>,
) -> impl IntoElement {
    let risk_color = permission.risk_level.color(theme);

    // Copy theme colors for move closures
    let success_bg_1 = theme.colors.success.opacity(0.1);
    let success_bg_2 = theme.colors.success.opacity(0.2);
    let success_border_1 = theme.colors.success.opacity(0.3);
    let success_border_2 = theme.colors.success.opacity(0.5);
    let success_color = theme.colors.success;

    let error_bg_1 = theme.colors.error.opacity(0.1);
    let error_bg_2 = theme.colors.error.opacity(0.2);
    let error_border_1 = theme.colors.error.opacity(0.3);
    let error_border_2 = theme.colors.error.opacity(0.5);
    let error_color = theme.colors.error;

    // Extract listeners before div chains
    let approve_listener = cx.listener(move |this, _, _window, cx| {
        this.approve_permission(idx, cx);
    });

    let deny_listener = cx.listener(move |this, _, _window, cx| {
        this.deny_permission(idx, cx);
    });

    div()
        .id(SharedString::from(format!("permission-{}", idx)))
        .px_4()
        .py_3()
        .border_b_1()
        .border_color(theme.colors.border.opacity(0.5))
        .flex()
        .flex_col()
        .gap_2()
        // Tool with icon and description
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        // Tool icon
                        .child(
                            div()
                                .text_base()
                                .child(get_tool_icon(&permission.tool))
                        )
                        // Tool name badge
                        .child(
                            div()
                                .px_2()
                                .py_px()
                                .rounded_sm()
                                .bg(theme.colors.accent.opacity(0.1))
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.colors.accent)
                                .child(permission.tool.clone())
                        )
                        // Tool description
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .child(get_tool_description(&permission.tool))
                        )
                )
                // Risk level badge
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .px_2()
                        .py_px()
                        .rounded_sm()
                        .bg(risk_color.opacity(0.1))
                        .text_xs()
                        .text_color(risk_color)
                        .child(permission.risk_level.icon())
                        .child(permission.risk_level.label())
                )
        )
        // Action description
        .child(
            div()
                .text_sm()
                .text_color(theme.colors.text)
                .child(permission.description.clone())
        )
        // Action/command
        .child(
            div()
                .px_2()
                .py_1()
                .rounded_sm()
                .bg(theme.colors.background)
                .border_1()
                .border_color(theme.colors.border)
                .text_xs()
                .font_family("monospace")
                .text_color(theme.colors.text_muted)
                .overflow_x_hidden()
                .child(permission.action.clone())
        )
        // Action buttons
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .mt_1()
                // Approve button
                .child(
                    div()
                        .id(SharedString::from(format!("approve-{}", idx)))
                        .flex_1()
                        .flex()
                        .items_center()
                        .justify_center()
                        .gap_1()
                        .px_3()
                        .py(px(6.0))
                        .rounded_md()
                        .bg(success_bg_1)
                        .border_1()
                        .border_color(success_border_1)
                        .cursor_pointer()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(success_color)
                        .hover(move |s| {
                            s.bg(success_bg_2)
                                .border_color(success_border_2)
                        })
                        .on_click(approve_listener)
                        .child("✓")
                        .child("Approve")
                )
                // Deny button
                .child(
                    div()
                        .id(SharedString::from(format!("deny-{}", idx)))
                        .flex_1()
                        .flex()
                        .items_center()
                        .justify_center()
                        .gap_1()
                        .px_3()
                        .py(px(6.0))
                        .rounded_md()
                        .bg(error_bg_1)
                        .border_1()
                        .border_color(error_border_1)
                        .cursor_pointer()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(error_color)
                        .hover(move |s| {
                            s.bg(error_bg_2)
                                .border_color(error_border_2)
                        })
                        .on_click(deny_listener)
                        .child("×")
                        .child("Deny")
                )
        )
}
