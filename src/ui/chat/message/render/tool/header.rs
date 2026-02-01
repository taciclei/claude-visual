//! Tool header and action buttons rendering

use gpui::prelude::*;
use gpui::*;

use super::super::super::types::MessageViewEvent;
use super::super::super::view::MessageView;
use crate::app::theme::Theme;

/// Get contextual skill suggestion based on tool name
fn get_skill_for_tool(tool_name: &str) -> Option<(&'static str, &'static str, &'static str)> {
    let name_lower = tool_name.to_lowercase();
    match name_lower.as_str() {
        "bash" => Some(("📖", "Explain", "/explain")),
        "read" => Some(("📖", "Explain", "/explain")),
        "write" | "edit" => Some(("👀", "Review", "/review")),
        "grep" | "glob" => Some(("🔍", "Explore", "/explore")),
        "task" => Some(("⚡", "APEX", "/apex")),
        "websearch" | "webfetch" | "web_search" | "web_fetch" => {
            Some(("💡", "Brainstorm", "/brainstorm"))
        }
        _ => None,
    }
}

/// Render the tool header with icon, name, description, and status badge
pub(super) fn render_tool_header(
    tool_icon_str: &str,
    tool_label: &str,
    tool_desc: &str,
    is_tool_result: bool,
    is_error: bool,
    border_color: Hsla,
    theme: &Theme,
) -> Div {
    // Clone strings for move closures
    let tool_icon_owned = tool_icon_str.to_string();
    let tool_label_owned = tool_label.to_string();
    let tool_desc_owned = tool_desc.to_string();
    let text_muted = theme.colors.text_muted;
    let error_color = theme.colors.error;
    let success_color = theme.colors.success;

    div()
        .flex()
        .items_center()
        .gap_2()
        // Tool icon and name
        .child(
            div()
                .flex()
                .items_center()
                .gap_1()
                .px_2()
                .py_px()
                .rounded_sm()
                .bg(border_color.opacity(0.1))
                .child(div().text_xs().child(tool_icon_owned))
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(border_color)
                        .child(tool_label_owned),
                ),
        )
        // Tool description
        .when(!tool_desc.is_empty(), move |d| {
            d.child(
                div()
                    .text_xs()
                    .text_color(text_muted)
                    .child(tool_desc_owned.clone()),
            )
        })
        // Status badge for results
        .when(is_tool_result, move |d| {
            d.child(
                div()
                    .px_2()
                    .py_px()
                    .rounded_sm()
                    .bg(if is_error {
                        error_color.opacity(0.1)
                    } else {
                        success_color.opacity(0.1)
                    })
                    .text_xs()
                    .text_color(if is_error { error_color } else { success_color })
                    .child(if is_error { "Failed" } else { "Success" }),
            )
        })
}

/// Render action buttons (open, rerun, copy, skill suggestion)
pub(super) fn render_actions(
    file_path: Option<String>,
    command: Option<String>,
    pattern_str: Option<String>,
    content: &str,
    theme: &Theme,
    cx: &mut Context<MessageView>,
) -> Div {
    // Copy theme colors for move closures
    let accent_color = theme.colors.accent;
    let warning_color = theme.colors.warning;
    let text_muted_color = theme.colors.text_muted;
    let text_color = theme.colors.text;
    let surface_hover_color = theme.colors.surface_hover;
    let info_color = theme.colors.info;

    // Determine tool name from context
    let tool_name = if file_path.is_some() {
        "read"
    } else if command.is_some() {
        "bash"
    } else if pattern_str.is_some() {
        "grep"
    } else {
        ""
    };

    // Get skill suggestion based on tool
    let skill_suggestion = get_skill_for_tool(tool_name);

    // Extract listeners before div builder
    let open_file_listener = file_path.as_ref().map(|path| {
        let path_clone = path.clone();
        cx.listener(move |_this, _, _window, cx| {
            cx.emit(MessageViewEvent::OpenFile(path_clone.clone()));
        })
    });

    let rerun_command_listener = command.as_ref().map(|cmd| {
        let cmd_clone = cmd.clone();
        cx.listener(move |_this, _, _window, cx| {
            cx.emit(MessageViewEvent::RerunCommand(cmd_clone.clone()));
        })
    });

    let copy_content = file_path
        .clone()
        .or(command.clone())
        .or(pattern_str.clone())
        .unwrap_or_else(|| content.to_string());

    let copy_listener = cx.listener(move |_this, _, _window, cx| {
        cx.write_to_clipboard(ClipboardItem::new_string(copy_content.clone()));
        tracing::info!("Tool content copied to clipboard");
    });

    div()
        .flex()
        .items_center()
        .gap_1()
        // Skill suggestion button
        .when_some(skill_suggestion, |d, (icon, label, cmd)| {
            let cmd_str = cmd.to_string();
            d.child(
                div()
                    .id("tool-skill-btn")
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_2()
                    .py_0p5()
                    .rounded_md()
                    .text_xs()
                    .bg(info_color.opacity(0.1))
                    .text_color(info_color)
                    .hover(move |s| s.bg(info_color.opacity(0.2)))
                    .cursor_pointer()
                    .on_click(cx.listener(move |_this, _, _window, cx| {
                        cx.emit(MessageViewEvent::ExecuteSkill(cmd_str.clone()));
                    }))
                    .child(icon)
                    .child(label),
            )
        })
        // Open file button (for file operations)
        .when_some(open_file_listener, |d, listener| {
            d.child(
                div()
                    .id("tool-open-btn")
                    .px_2()
                    .py_0p5()
                    .rounded_sm()
                    .text_xs()
                    .text_color(accent_color)
                    .hover(move |s| s.bg(accent_color.opacity(0.1)))
                    .cursor_pointer()
                    .on_click(listener)
                    .child("Open"),
            )
        })
        // Rerun button (for commands)
        .when_some(rerun_command_listener, |d, listener| {
            d.child(
                div()
                    .id("tool-rerun-btn")
                    .px_2()
                    .py_0p5()
                    .rounded_sm()
                    .text_xs()
                    .text_color(warning_color)
                    .hover(move |s| s.bg(warning_color.opacity(0.1)))
                    .cursor_pointer()
                    .on_click(listener)
                    .child("Run again"),
            )
        })
        // Copy button
        .child(
            div()
                .id("tool-copy-btn")
                .px_2()
                .py_0p5()
                .rounded_sm()
                .text_xs()
                .text_color(text_muted_color)
                .hover(move |s| s.bg(surface_hover_color).text_color(text_color))
                .cursor_pointer()
                .on_click(copy_listener)
                .child("Copy"),
        )
}
