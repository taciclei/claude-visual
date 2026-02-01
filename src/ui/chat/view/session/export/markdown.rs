//! Markdown export format

use crate::claude::message::MessageRole;

use super::super::super::core::ChatView;

impl ChatView {
    /// Export conversation to Markdown format
    pub(crate) fn export_to_markdown(&self) -> String {
        let mut md = String::new();
        let stats = self.calculate_stats();

        // Header with title
        let title = self.display_title();
        md.push_str(&format!("# {}\n\n", title));

        // Metadata table
        md.push_str("## Conversation Info\n\n");
        md.push_str("| Property | Value |\n");
        md.push_str("|----------|-------|\n");

        if let Some(ref conv_id) = self.current_conversation_id {
            md.push_str(&format!("| Conversation ID | `{}` |\n", conv_id));
        }
        if let Some(ref info) = self.session_info {
            if !info.session_id.is_empty() {
                md.push_str(&format!("| Session ID | `{}` |\n", info.session_id));
            }
            if !info.model.is_empty() {
                md.push_str(&format!("| Model | {} |\n", info.model));
            }
            if !info.version.is_empty() {
                md.push_str(&format!("| Claude Code Version | {} |\n", info.version));
            }
            if !info.cwd.is_empty() {
                md.push_str(&format!("| Working Directory | `{}` |\n", info.cwd));
            }
        }
        if !self.messages.is_empty() {
            if let Some(first) = self.messages.first() {
                md.push_str(&format!("| Started | {} |\n", first.timestamp.format("%Y-%m-%d %H:%M:%S UTC")));
            }
            if let Some(last) = self.messages.last() {
                md.push_str(&format!("| Last Message | {} |\n", last.timestamp.format("%Y-%m-%d %H:%M:%S UTC")));
            }
        }
        md.push_str(&format!("| Duration | {} |\n", stats.format_duration()));
        md.push_str(&format!("| Total Messages | {} |\n", stats.message_count));
        md.push_str(&format!("| User Messages | {} |\n", stats.user_message_count));
        md.push_str(&format!("| Assistant Messages | {} |\n", stats.assistant_message_count));
        md.push_str(&format!("| Tool Calls | {} |\n", stats.tool_use_count));
        md.push_str(&format!("| Total Words | {} |\n", stats.format_words()));
        md.push_str(&format!("| Estimated Tokens | ~{} |\n", stats.format_tokens()));

        // Cost info if available
        if self.stats.cost > 0.0 {
            md.push_str(&format!("| Session Cost | ${:.4} |\n", self.stats.cost));
        }
        if self.stats.input_tokens > 0 || self.stats.output_tokens > 0 {
            md.push_str(&format!("| Input Tokens | {} |\n", Self::format_token_count(self.stats.input_tokens)));
            md.push_str(&format!("| Output Tokens | {} |\n", Self::format_token_count(self.stats.output_tokens)));
        }

        md.push_str("\n---\n\n");
        md.push_str("## Messages\n\n");

        // Messages
        for message in &self.messages {
            let role_header = match message.role {
                MessageRole::User => "## User",
                MessageRole::Assistant => "## Assistant",
                MessageRole::ToolUse => "## Tool Use",
                MessageRole::ToolResult => "## Tool Result",
                MessageRole::Error => "## Error",
                MessageRole::Thinking => "## Thinking",
                MessageRole::System => "## System",
            };

            md.push_str(role_header);
            md.push_str("\n\n");

            // Timestamp
            md.push_str(&format!("*{}*\n\n", message.timestamp.format("%H:%M:%S")));

            // Tool name if applicable
            if let Some(ref tool) = message.tool_name {
                md.push_str(&format!("**Tool:** `{}`\n\n", tool));
            }

            // Error indicator
            if message.is_error {
                md.push_str("> **Error**\n\n");
            }

            // Content - detect code blocks and preserve them
            let content = &message.content;
            if message.role == MessageRole::ToolUse || message.role == MessageRole::ToolResult {
                // Tool messages are often JSON/code, wrap in code block
                md.push_str("```\n");
                md.push_str(content);
                if !content.ends_with('\n') {
                    md.push('\n');
                }
                md.push_str("```\n");
            } else {
                // Regular content, preserve existing code blocks
                md.push_str(content);
                if !content.ends_with('\n') {
                    md.push('\n');
                }
            }

            md.push_str("\n---\n\n");
        }

        md
    }
}
