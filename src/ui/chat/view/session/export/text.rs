//! Plain text export format

use crate::claude::message::MessageRole;

use super::super::super::core::ChatView;

impl ChatView {
    /// Export conversation to plain text format
    pub(crate) fn export_to_plain_text(&self) -> String {
        let mut text = String::new();
        let title = self.display_title();
        let stats = self.calculate_stats();

        text.push_str(&format!("{}\n", title));
        text.push_str(&"=".repeat(title.len()));
        text.push_str("\n\n");

        if self.export.include_metadata {
            text.push_str(&format!("Messages: {}\n", stats.message_count));
            text.push_str(&format!("Duration: {}\n", stats.format_duration()));
            text.push_str(&format!("Words: {}\n", stats.format_words()));
            if let Some(ref info) = self.session_info {
                if !info.model.is_empty() {
                    text.push_str(&format!("Model: {}\n", info.model));
                }
            }
            text.push_str("\n");
            text.push_str(&"-".repeat(40));
            text.push_str("\n\n");
        }

        for message in &self.messages {
            // Filter based on export settings
            if !self.export.include_tools && matches!(message.role, MessageRole::ToolUse | MessageRole::ToolResult) {
                continue;
            }
            if !self.export.include_thinking && matches!(message.role, MessageRole::Thinking) {
                continue;
            }

            let role_name = match message.role {
                MessageRole::User => "YOU",
                MessageRole::Assistant => "CLAUDE",
                MessageRole::ToolUse => "TOOL",
                MessageRole::ToolResult => "RESULT",
                MessageRole::Error => "ERROR",
                MessageRole::Thinking => "THINKING",
                MessageRole::System => "SYSTEM",
            };

            text.push_str(&format!("[{}] {}\n", role_name, message.timestamp.format("%H:%M:%S")));
            if let Some(ref tool) = message.tool_name {
                text.push_str(&format!("Tool: {}\n", tool));
            }
            text.push_str(&message.content);
            if !message.content.ends_with('\n') {
                text.push('\n');
            }
            text.push_str("\n---\n\n");
        }

        text
    }
}
