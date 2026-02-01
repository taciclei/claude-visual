//! Message operation methods for ChatView

use gpui::*;
use super::core::ChatView;
use crate::claude::message::MessageRole;
use super::types::NotificationType;

impl ChatView {
    /// Get the number of messages in the conversation
    pub fn messages_len(&self) -> usize {
        self.messages.len()
    }

    /// Check if there are messages to export
    pub fn has_messages(&self) -> bool {
        !self.messages.is_empty()
    }

    /// Edit the last user message (puts content back in input)
    pub fn edit_last_message(&mut self, cx: &mut Context<Self>) {
        if self.streaming.is_streaming {
            self.show_notification(
                "Cannot edit while streaming".to_string(),
                NotificationType::Warning,
                cx
            );
            return;
        }

        if let Some(content) = self.get_last_user_message() {
            let content = content.to_string();
            // Set the input to the last message content
            self.input.update(cx, |input, cx| {
                input.set_text(content.clone(), cx);
            });
            self.show_notification(
                "Editing last message".to_string(),
                NotificationType::Info,
                cx
            );
        } else {
            self.show_notification(
                "No message to edit".to_string(),
                NotificationType::Warning,
                cx
            );
        }
    }

    /// Copy the last assistant response to clipboard
    pub fn copy_last_response(&mut self, cx: &mut Context<Self>) {
        if let Some(content) = self.get_last_assistant_message() {
            let content = content.to_string();
            cx.write_to_clipboard(ClipboardItem::new_string(content.clone()));
            let word_count = content.split_whitespace().count();
            self.show_notification(
                format!("Last response copied ({} words)", word_count),
                NotificationType::Success,
                cx
            );
            tracing::info!("Last response copied to clipboard ({} words)", word_count);
        } else {
            self.show_notification(
                "No assistant response to copy".to_string(),
                NotificationType::Warning,
                cx
            );
        }
    }

    /// Branch conversation from a specific message index
    /// This removes all messages after the specified index and puts the message content in input for editing
    pub fn branch_from_message(&mut self, message_index: usize, cx: &mut Context<Self>) {
        if self.streaming.is_streaming {
            self.show_notification("Cannot branch while streaming", NotificationType::Warning, cx);
            return;
        }

        if message_index >= self.messages.len() {
            self.show_notification("Invalid message index", NotificationType::Error, cx);
            return;
        }

        let message = &self.messages[message_index];

        // Only allow branching from user messages
        if message.role != MessageRole::User {
            self.show_notification("Can only branch from your messages", NotificationType::Warning, cx);
            return;
        }

        let content = message.content.clone();

        // Remove messages from this point onwards
        let removed_count = self.messages.len() - message_index;
        self.messages.truncate(message_index);
        self.message_views.truncate(message_index);

        // Put content in input for editing
        self.input.update(cx, |input, cx| {
            input.set_text(content, cx);
        });

        self.show_notification(
            format!("Branching from message (removed {} messages)", removed_count),
            NotificationType::Info,
            cx
        );
        cx.notify();
    }

    /// Copy entire conversation to clipboard as plain text
    pub fn copy_conversation_to_clipboard(&mut self, cx: &mut Context<Self>) {
        if self.messages.is_empty() {
            return;
        }

        let mut text = String::new();

        for message in &self.messages {
            let role_label = match message.role {
                MessageRole::User => "You",
                MessageRole::Assistant => "Claude",
                MessageRole::ToolUse => "Tool",
                MessageRole::ToolResult => "Result",
                MessageRole::Error => "Error",
                MessageRole::Thinking => "Thinking",
                MessageRole::System => "System",
            };

            text.push_str(&format!("[{}]\n", role_label));

            if let Some(ref tool) = message.tool_name {
                text.push_str(&format!("({tool})\n"));
            }

            text.push_str(&message.content);
            text.push_str("\n\n");
        }

        cx.write_to_clipboard(ClipboardItem::new_string(text));
        let count = self.messages.len();
        self.show_notification(
            format!("Conversation copied ({} messages)", count),
            NotificationType::Success,
            cx
        );
        tracing::info!("Conversation copied to clipboard ({} messages)", count);
    }

    /// Copy conversation to clipboard as Markdown
    pub fn copy_conversation_as_markdown(&mut self, cx: &mut Context<Self>) {
        if self.messages.is_empty() {
            return;
        }

        let markdown = self.export_to_markdown();
        let count = self.messages.len();
        cx.write_to_clipboard(ClipboardItem::new_string(markdown));
        self.show_notification(
            format!("Conversation copied as Markdown ({} messages)", count),
            NotificationType::Success,
            cx
        );
        tracing::info!("Conversation copied as Markdown ({} messages)", count);
    }

    // ==================== Export Placeholder Methods ====================
    // These are placeholders left for backwards compatibility.
    // The actual implementations have been moved to session.rs

    /// Export conversation to Markdown format - MOVED TO session.rs
    fn export_to_markdown_placeholder(&self) -> String {
        // This method has been moved to session.rs
        // TODO: Remove this placeholder after verifying all references are updated
        String::new()
    }

    /// Export conversation to JSON format - MOVED TO session.rs
    fn export_to_json_placeholder(&self) -> String {
        String::new()
    }

    /// Export conversation to HTML format - MOVED TO session.rs
    fn export_to_html_placeholder(&self) -> String {
        String::new()
    }

    /// Export conversation to plain text format - MOVED TO session.rs
    fn export_to_plain_text_placeholder(&self) -> String {
        String::new()
    }

    /// Export with current settings - MOVED TO session.rs
    fn export_with_format_placeholder(&self) -> String {
        String::new()
    }

    /// PLACEHOLDER - Metadata table for export
    fn _export_metadata_placeholder(&self) -> String {
        String::new()
    }
}
