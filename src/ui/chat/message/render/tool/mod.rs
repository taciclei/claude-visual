//! Tool message rendering

mod types;
mod parser;
mod content;
mod header;

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;
use crate::claude::message::MessageRole;
use super::super::view::MessageView;
use super::super::utils::{tool_icon, tool_description};

impl MessageView {
    pub(in crate::ui::chat::message) fn render_tool_content(&self, theme: &Theme, cx: &mut Context<Self>) -> Div {
        let content = self.message.content.clone();
        let is_tool_result = self.message.role == MessageRole::ToolResult;
        let is_error = self.message.is_error;
        let tool_name = self.message.tool_name.clone();

        // Parse content into structured display
        let tool_display = parser::parse_tool_content(&content, is_tool_result);

        // Get tool icon and name display
        let (tool_icon_str, tool_label) = if let Some(ref name) = tool_name {
            (tool_icon(name), name.clone())
        } else if is_tool_result {
            ("✓", "Result".to_string())
        } else {
            ("🔧", "Tool".to_string())
        };

        // Get tool description
        let tool_desc = tool_name.as_ref().map(|n| tool_description(n)).unwrap_or("");

        // Border color based on tool result status
        let border_color = if is_error {
            theme.colors.error
        } else if is_tool_result {
            theme.colors.success
        } else {
            theme.colors.info
        };

        // Extract action data
        let file_path = tool_display.file_path();
        let command = tool_display.command();
        let pattern_str = tool_display.pattern();

        // Render content element
        let content_element = content::render_content(&tool_display, theme, is_error);

        // Render tool header
        let header_left = header::render_tool_header(
            &tool_icon_str,
            &tool_label,
            tool_desc,
            is_tool_result,
            is_error,
            border_color,
            theme,
        );

        // Render action buttons
        let actions = header::render_actions(
            file_path,
            command,
            pattern_str,
            &content,
            theme,
            cx,
        );

        div().child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .px_3()
                .py_2()
                .rounded_md()
                .bg(theme.colors.background)
                .border_l_2()
                .border_color(border_color.opacity(0.5))
                // Tool header with name badge, status, and actions
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .mb_1()
                        .child(header_left)
                        .child(actions)
                )
                // Content
                .child(content_element)
        )
    }
}
