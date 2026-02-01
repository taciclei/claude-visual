//! Message reactions, context menu, and session notes functionality

use gpui::prelude::*;
use gpui::*;

use super::core::ChatView;
use super::types::{ChatViewEvent, ContextMenuState, MessageReaction, NotificationType};
use crate::claude::message::MessageRole;

impl ChatView {
    // ==================== Message Reactions ====================

    /// Add a reaction to a message
    pub(crate) fn add_reaction(
        &mut self,
        message_index: usize,
        emoji: &'static str,
        cx: &mut Context<Self>,
    ) {
        let reactions = self
            .message_reactions
            .entry(message_index)
            .or_insert_with(Vec::new);

        // Check if already reacted with this emoji
        if reactions.iter().any(|r| r.emoji == emoji) {
            // Remove if already exists (toggle behavior)
            reactions.retain(|r| r.emoji != emoji);
            if reactions.is_empty() {
                self.message_reactions.remove(&message_index);
            }
        } else {
            reactions.push(MessageReaction::new(emoji));
        }
        cx.notify();
    }

    /// Get reactions for a message
    pub(crate) fn get_reactions(&self, message_index: usize) -> Option<&Vec<MessageReaction>> {
        self.message_reactions.get(&message_index)
    }

    /// Check if message has reactions
    pub(crate) fn has_reactions(&self, message_index: usize) -> bool {
        self.message_reactions
            .get(&message_index)
            .map(|r| !r.is_empty())
            .unwrap_or(false)
    }

    /// Show quick reaction picker for a message
    pub(crate) fn show_quick_reactions(&mut self, message_index: usize, cx: &mut Context<Self>) {
        self.quick_action_message = Some(message_index);
        cx.notify();
    }

    /// Hide quick reaction picker
    pub(crate) fn hide_quick_reactions(&mut self, cx: &mut Context<Self>) {
        self.quick_action_message = None;
        cx.notify();
    }

    // ==================== Context Menu ====================

    /// Show context menu at position
    pub(crate) fn show_context_menu(
        &mut self,
        message_index: usize,
        x: f32,
        y: f32,
        cx: &mut Context<Self>,
    ) {
        self.context_menu = Some(ContextMenuState::new(message_index, x, y));
        cx.notify();
    }

    /// Hide context menu
    pub(crate) fn hide_context_menu(&mut self, cx: &mut Context<Self>) {
        self.context_menu = None;
        cx.notify();
    }

    /// Execute context menu action
    pub(crate) fn execute_context_menu_action(&mut self, action: &str, cx: &mut Context<Self>) {
        let message_index = match &self.context_menu {
            Some(menu) => menu.message_index,
            None => return,
        };

        match action {
            "copy" => {
                // Copy message to clipboard
                if let Some(msg) = self.messages.get(message_index) {
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
                        format!("Message copied to clipboard"),
                        NotificationType::Success,
                        cx,
                    );
                }
            }
            "pin" => self.toggle_pin(message_index, cx),
            "bookmark" => self.toggle_bookmark(message_index, cx),
            "react" => self.show_quick_reactions(message_index, cx),
            "edit" => {
                // Edit user messages
                if let Some(msg) = self.messages.get(message_index) {
                    if matches!(msg.role, MessageRole::User) {
                        // Set input to message content for editing
                        let content = msg.content.clone();
                        self.input.update(cx, |input, cx| {
                            input.set_text(content, cx);
                        });
                    }
                }
            }
            "delete" => {
                // Remove message from list
                if message_index < self.messages.len() {
                    self.messages.remove(message_index);
                    if message_index < self.message_views.len() {
                        self.message_views.remove(message_index);
                    }
                    self.show_notification(
                        "Message deleted".to_string(),
                        NotificationType::Info,
                        cx,
                    );
                }
            }
            "regenerate" => {
                // Regenerate from this point
                if let Some(msg) = self.messages.get(message_index) {
                    if matches!(msg.role, MessageRole::User) {
                        let prompt = msg.content.clone();
                        // Truncate conversation up to this message
                        self.messages.truncate(message_index);
                        self.message_views.truncate(message_index);
                        // Re-submit
                        cx.emit(ChatViewEvent::Submit(prompt));
                    }
                }
            }
            "branch" => {
                self.branch_from_message(message_index, cx);
            }
            "retry_from_here" => {
                self.retry_from_message(message_index, cx);
            }
            "quote" => {
                self.quote_message(message_index, cx);
            }
            "copy_as_markdown" => {
                if let Some(msg) = self.messages.get(message_index) {
                    let role = match msg.role {
                        MessageRole::User => "**You:**",
                        MessageRole::Assistant => "**Claude:**",
                        MessageRole::ToolUse => "**Tool Use:**",
                        MessageRole::ToolResult => "**Tool Result:**",
                        MessageRole::Error => "**Error:**",
                        MessageRole::Thinking => "**Thinking:**",
                        MessageRole::System => "**System:**",
                    };
                    let markdown = format!("{}\n\n{}", role, msg.content);
                    cx.write_to_clipboard(ClipboardItem::new_string(markdown));
                    self.show_notification("Copied as Markdown", NotificationType::Success, cx);
                }
            }
            // Claude Code skill actions
            "skill_explain" => {
                if let Some(msg) = self.messages.get(message_index) {
                    let prompt = format!("/explain {}", Self::extract_code_block(&msg.content));
                    cx.emit(ChatViewEvent::Submit(prompt));
                }
            }
            "skill_review" => {
                if let Some(msg) = self.messages.get(message_index) {
                    let prompt = format!("/review {}", Self::extract_code_block(&msg.content));
                    cx.emit(ChatViewEvent::Submit(prompt));
                }
            }
            "skill_refactor" => {
                if let Some(msg) = self.messages.get(message_index) {
                    let prompt = format!("/refactor {}", Self::extract_code_block(&msg.content));
                    cx.emit(ChatViewEvent::Submit(prompt));
                }
            }
            "skill_tests" => {
                if let Some(msg) = self.messages.get(message_index) {
                    let prompt = format!(
                        "Generate unit tests for this code:\n\n{}",
                        Self::extract_code_block(&msg.content)
                    );
                    cx.emit(ChatViewEvent::Submit(prompt));
                }
            }
            "continue" => {
                cx.emit(ChatViewEvent::Submit("Continue".to_string()));
            }
            "explain_more" => {
                if let Some(msg) = self.messages.get(message_index) {
                    let preview = if msg.content.len() > 100 {
                        format!("{}...", &msg.content[..100])
                    } else {
                        msg.content.clone()
                    };
                    let prompt = format!("Please explain this in more detail: {}", preview);
                    cx.emit(ChatViewEvent::Submit(prompt));
                }
            }
            "separator" => {
                // Do nothing for separator
            }
            _ => {}
        }

        self.hide_context_menu(cx);
    }

    // ==================== Session Notes ====================

    /// Toggle notes panel
    pub(crate) fn toggle_notes_panel(&mut self, cx: &mut Context<Self>) {
        self.panels.notes_panel = !self.panels.notes_panel;
        cx.notify();
    }

    /// Set session notes
    pub(crate) fn set_session_notes(&mut self, notes: impl Into<String>, cx: &mut Context<Self>) {
        self.session_notes = notes.into();
        cx.notify();
    }

    /// Get session notes
    pub(crate) fn get_session_notes(&self) -> &str {
        &self.session_notes
    }

    /// Check if session has notes
    pub(crate) fn has_notes(&self) -> bool {
        !self.session_notes.trim().is_empty()
    }

    /// Extract code block from message content
    fn extract_code_block(content: &str) -> String {
        // Try to extract the first code block
        if let Some(start) = content.find("```") {
            let rest = &content[start + 3..];
            // Skip language identifier
            let code_start = rest.find('\n').map(|i| i + 1).unwrap_or(0);
            if let Some(end) = rest[code_start..].find("```") {
                return rest[code_start..code_start + end].trim().to_string();
            }
        }
        // No code block found, return trimmed content
        content.trim().to_string()
    }
}
