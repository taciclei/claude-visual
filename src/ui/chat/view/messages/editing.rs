//! Message editing methods

use gpui::*;
use std::collections::HashSet;

use crate::claude::message::MessageRole;

use super::super::core::ChatView;
use super::super::types::{ChatViewEvent, NotificationType};

impl ChatView {
    /// Edit and resend a specific message (branch from that point)
    pub fn edit_message_at(&mut self, message_index: usize, cx: &mut Context<Self>) {
        self.branch_from_message(message_index, cx);
    }

    /// Retry from a specific message (re-send everything from that point)
    pub fn retry_from_message(&mut self, message_index: usize, cx: &mut Context<Self>) {
        if self.streaming.is_streaming {
            self.show_notification("Cannot retry while streaming", NotificationType::Warning, cx);
            return;
        }

        if message_index >= self.messages.len() {
            return;
        }

        let message = &self.messages[message_index];
        if message.role != MessageRole::User {
            self.show_notification("Can only retry from your messages", NotificationType::Warning, cx);
            return;
        }

        let content = message.content.clone();

        // Remove all messages from this point
        self.messages.truncate(message_index);
        self.message_views.truncate(message_index);

        // Re-submit the message
        cx.emit(ChatViewEvent::Submit(content));
        self.show_notification("Retrying from message...", NotificationType::Info, cx);
    }

    /// Delete a specific message
    pub fn delete_message_at(&mut self, message_index: usize, cx: &mut Context<Self>) {
        if message_index >= self.messages.len() {
            return;
        }

        self.messages.remove(message_index);
        self.message_views.remove(message_index);

        // Update selected message index
        if let Some(selected) = self.selected_message_index {
            if selected >= self.messages.len() {
                self.selected_message_index = if self.messages.is_empty() {
                    None
                } else {
                    Some(self.messages.len() - 1)
                };
            }
        }

        // Update bookmarks - remove the deleted one and adjust indices
        let mut new_bookmarks = HashSet::new();
        for &idx in &self.bookmarked_messages {
            if idx < message_index {
                new_bookmarks.insert(idx);
            } else if idx > message_index {
                new_bookmarks.insert(idx - 1);
            }
        }
        self.bookmarked_messages = new_bookmarks;

        // Same for pinned messages
        let mut new_pinned = HashSet::new();
        for &idx in &self.pinned_messages {
            if idx < message_index {
                new_pinned.insert(idx);
            } else if idx > message_index {
                new_pinned.insert(idx - 1);
            }
        }
        self.pinned_messages = new_pinned;

        self.show_notification("Message deleted", NotificationType::Info, cx);
        cx.notify();
    }

    /// Quote a message in the input (for replying with context)
    pub fn quote_message(&mut self, message_index: usize, cx: &mut Context<Self>) {
        if message_index >= self.messages.len() {
            return;
        }

        let message = &self.messages[message_index];
        let role_label = match message.role {
            MessageRole::User => "You",
            MessageRole::Assistant => "Claude",
            MessageRole::ToolUse => "Tool",
            MessageRole::ToolResult => "Result",
            _ => "Message",
        };

        // Create a quoted version (first 200 chars)
        let content = if message.content.len() > 200 {
            format!("{}...", &message.content[..200])
        } else {
            message.content.clone()
        };

        let quoted = format!("> [{}]: {}\n\n", role_label, content.replace('\n', "\n> "));

        // Append to current input
        self.input.update(cx, |input, cx| {
            let current = input.text();
            let new_text = if current.is_empty() {
                quoted
            } else {
                format!("{}\n{}", current, quoted)
            };
            input.set_text(new_text, cx);
        });

        self.show_notification("Message quoted in input", NotificationType::Info, cx);
    }
}
