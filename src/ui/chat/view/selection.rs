//! Message selection and navigation methods

use gpui::*;

use crate::claude::message::MessageRole;

use super::core::ChatView;
use super::types::NotificationType;

impl ChatView {
    /// Insert a mention into the chat input
    pub fn insert_mention(&mut self, mention: &str, cx: &mut Context<Self>) {
        self.input.update(cx, |input, cx| {
            input.insert_text(mention, cx);
        });
    }

    /// Increment unread count (called when new message arrives while scrolled away)
    pub fn increment_unread(&mut self, cx: &mut Context<Self>) {
        if self.show_scroll_to_bottom {
            self.unread_count += 1;
            cx.notify();
        }
    }

    /// Get unread message count
    pub fn unread_count(&self) -> usize {
        self.unread_count
    }

    /// Select first message
    pub fn select_first_message(&mut self, cx: &mut Context<Self>) {
        if !self.messages.is_empty() {
            self.selected_message_index = Some(0);
            self.update_message_selection_states(cx);
            cx.notify();
        }
    }

    /// Select last message
    pub fn select_last_message(&mut self, cx: &mut Context<Self>) {
        if !self.messages.is_empty() {
            self.selected_message_index = Some(self.messages.len() - 1);
            self.update_message_selection_states(cx);
            cx.notify();
        }
    }

    /// Clear message selection
    pub fn clear_message_selection(&mut self, cx: &mut Context<Self>) {
        self.selected_message_index = None;
        self.update_message_selection_states(cx);
        cx.notify();
    }

    /// Copy selected message to clipboard
    pub fn copy_selected_message(&mut self, cx: &mut Context<Self>) {
        if let Some(idx) = self.selected_message_index {
            if let Some(msg) = self.messages.get(idx) {
                let role_label = match msg.role {
                    MessageRole::User => "You",
                    MessageRole::Assistant => "Claude",
                    MessageRole::ToolUse => "Tool Use",
                    MessageRole::ToolResult => "Tool Result",
                    MessageRole::Error => "Error",
                    MessageRole::Thinking => "Thinking",
                    MessageRole::System => "System",
                };
                let text = format!("**{}:**\n{}", role_label, msg.content);
                cx.write_to_clipboard(ClipboardItem::new_string(text));
                self.show_notification(
                    format!("Message {} copied to clipboard", idx + 1),
                    NotificationType::Success,
                    cx
                );
                tracing::info!("Copied selected message {} to clipboard", idx + 1);
            }
        }
    }

    /// Toggle bookmark on selected message
    pub fn bookmark_selected_message(&mut self, cx: &mut Context<Self>) {
        if let Some(idx) = self.selected_message_index {
            if let Some(view) = self.message_views.get(idx) {
                view.update(cx, |v, cx| v.toggle_bookmark(cx));
                let is_bookmarked = view.read(cx).is_bookmarked();
                self.show_notification(
                    if is_bookmarked {
                        format!("Message {} bookmarked", idx + 1)
                    } else {
                        format!("Bookmark removed from message {}", idx + 1)
                    },
                    NotificationType::Info,
                    cx
                );
                tracing::info!(
                    "Message {} bookmark toggled: {}",
                    idx + 1,
                    if is_bookmarked { "added" } else { "removed" }
                );
                cx.notify();
            }
        }
    }

    /// Check if selected message is bookmarked
    pub fn is_selected_bookmarked(&self, cx: &Context<Self>) -> bool {
        self.selected_message_index
            .and_then(|idx| self.message_views.get(idx))
            .map(|view| view.read(cx).is_bookmarked())
            .unwrap_or(false)
    }

    /// Check if there is a selected message
    pub fn has_selected_message(&self) -> bool {
        self.selected_message_index.is_some()
    }

    /// Start editing title
    pub fn start_editing_title(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Initialize buffer with current title or display title
        self.title_edit_buffer = self.conversation_title.clone().unwrap_or_else(|| {
            if let Some(first_user_msg) = self.messages.iter().find(|m| m.role == MessageRole::User) {
                let content = first_user_msg.content.trim();
                if content.len() > 50 {
                    content[..47].to_string()
                } else {
                    content.to_string()
                }
            } else {
                String::new()
            }
        });
        self.editing_title = true;
        self.title_focus.focus(window);
        cx.notify();
    }

    /// Cancel editing title (discard changes)
    pub fn cancel_editing_title(&mut self, cx: &mut Context<Self>) {
        self.editing_title = false;
        self.title_edit_buffer.clear();
        cx.notify();
    }

    /// Save the edited title
    pub fn save_edited_title(&mut self, cx: &mut Context<Self>) {
        let title = self.title_edit_buffer.trim().to_string();
        self.conversation_title = if title.is_empty() { None } else { Some(title) };
        self.editing_title = false;
        self.title_edit_buffer.clear();
        cx.notify();
    }

    /// Get the title edit buffer
    pub fn title_edit_buffer(&self) -> &str {
        &self.title_edit_buffer
    }

    /// Set the title edit buffer
    pub fn set_title_edit_buffer(&mut self, text: String, cx: &mut Context<Self>) {
        self.title_edit_buffer = text;
        cx.notify();
    }

    /// Check if title is being edited
    pub fn is_editing_title(&self) -> bool {
        self.editing_title
    }
}
