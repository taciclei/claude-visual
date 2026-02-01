//! Conversation management methods

use gpui::*;

use crate::claude::message::{ClaudeMessage, MessageRole};
use crate::storage::models::{Conversation, Message};

use super::super::core::ChatView;
use super::super::types::{ConnectionStatus, MessageFilter, NotificationType};

impl ChatView {
    /// Get current conversation ID, creating one if needed
    fn ensure_conversation(&mut self) -> String {
        if let Some(ref id) = self.current_conversation_id {
            return id.clone();
        }

        // Create new conversation
        let project_id = self.app_state.current_directory().and_then(|_path| {
            // Try to find matching project
            // For now, just use None
            None::<String>
        });

        let title = format!(
            "Conversation {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M")
        );
        let conversation = Conversation::new(title, project_id);
        let conv_id = conversation.id.clone();

        // Save to database
        if let Err(e) = self.app_state.database.insert_conversation(&conversation) {
            tracing::error!("Failed to save conversation: {}", e);
        }

        self.current_conversation_id = Some(conv_id.clone());
        conv_id
    }

    /// Save a message to the database
    pub(crate) fn save_message(&self, message: &ClaudeMessage) {
        let conv_id = match &self.current_conversation_id {
            Some(id) => id.clone(),
            None => return,
        };

        let role = match message.role {
            MessageRole::User => "user",
            MessageRole::Assistant => "assistant",
            MessageRole::ToolUse => "tool_use",
            MessageRole::ToolResult => "tool_result",
            MessageRole::Error => "error",
            MessageRole::Thinking => "thinking",
            MessageRole::System => "system",
        };

        let db_message = Message {
            id: uuid::Uuid::new_v4().to_string(),
            conversation_id: conv_id,
            role: role.to_string(),
            content: message.content.clone(),
            tool_name: message.tool_name.clone(),
            is_error: message.is_error,
            timestamp: message.timestamp,
        };

        if let Err(e) = self.app_state.database.insert_message(&db_message) {
            tracing::error!("Failed to save message: {}", e);
        }
    }

    /// Load a conversation by ID
    pub fn load_conversation(&mut self, conversation_id: &str, cx: &mut Context<Self>) {
        // Clear current state
        self.messages.clear();
        self.message_views.clear();
        self.streaming.current_message = None;
        self.streaming_message_view = None;
        self.streaming.is_streaming = false;
        self.current_conversation_id = Some(conversation_id.to_string());

        // Load messages from database
        match self.app_state.database.get_messages(conversation_id) {
            Ok(db_messages) => {
                for db_msg in db_messages {
                    let role = match db_msg.role.as_str() {
                        "user" => MessageRole::User,
                        "assistant" => MessageRole::Assistant,
                        "tool_use" => MessageRole::ToolUse,
                        "tool_result" => MessageRole::ToolResult,
                        "error" => MessageRole::Error,
                        _ => MessageRole::User,
                    };

                    let message = ClaudeMessage {
                        role,
                        content: db_msg.content,
                        timestamp: db_msg.timestamp,
                        tool_name: db_msg.tool_name,
                        is_error: db_msg.is_error,
                    };

                    // Create entity for this message
                    let view = self.create_message_view(message.clone(), cx);
                    self.message_views.push(view);
                    self.messages.push(message);
                }
                tracing::info!(
                    "Loaded {} messages from conversation {}",
                    self.messages.len(),
                    conversation_id
                );
            }
            Err(e) => {
                tracing::error!("Failed to load conversation: {}", e);
            }
        }

        cx.notify();
    }

    /// Add a message to the chat
    pub fn add_message(&mut self, message: ClaudeMessage, cx: &mut Context<Self>) {
        // Ensure we have a conversation (creates one on first message)
        self.ensure_conversation();

        // Save to database
        self.save_message(&message);

        // Create entity and add to local state
        let view = self.create_message_view(message.clone(), cx);
        self.message_views.push(view);
        self.messages.push(message);

        // Auto-scroll to show new message if enabled
        if self.auto_scroll {
            self.show_scroll_to_bottom = false;
            self.unread_count = 0;
        }

        cx.notify();
    }

    /// Clear the current conversation
    pub fn clear_conversation(&mut self, cx: &mut Context<Self>) {
        // Stop any streaming first
        if self.streaming.is_streaming {
            self.request_stop(cx);
        }

        // Clear all messages
        self.messages.clear();
        self.message_views.clear();
        self.streaming_message_view = None;
        self.streaming.current_message = None;
        self.current_conversation_id = None;

        // Reset search state
        self.search.query.clear();
        self.search.results.clear();
        self.search.current_result = 0;
        self.search.show = false;

        // Reset scroll state
        self.show_scroll_to_bottom = false;
        self.unread_count = 0;

        // Reset filter
        self.message_filter = MessageFilter::All;

        // Reset title
        self.conversation_title = None;
        self.editing_title = false;
        self.title_edit_buffer.clear();

        // Reset selection
        self.selected_message_index = None;

        tracing::info!("Conversation cleared");
        cx.notify();
    }

    /// Clear conversation but keep conversation ID (for continue from history)
    pub fn clear_messages(&mut self, cx: &mut Context<Self>) {
        if self.streaming.is_streaming {
            self.request_stop(cx);
        }

        self.messages.clear();
        self.message_views.clear();
        self.streaming_message_view = None;
        self.streaming.current_message = None;

        self.search.query.clear();
        self.search.results.clear();
        self.search.current_result = 0;

        self.show_scroll_to_bottom = false;
        self.unread_count = 0;
        self.message_filter = MessageFilter::All;

        cx.notify();
    }

    /// Clear all messages and start a new conversation
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        // Show session summary if there were messages
        if !self.messages.is_empty() {
            let summary = self.get_brief_summary();
            self.show_notification(
                format!("Session ended: {}", summary),
                NotificationType::Info,
                cx,
            );
        }

        self.messages.clear();
        self.message_views.clear();
        self.streaming.current_message = None;
        self.streaming_message_view = None;
        self.streaming.is_streaming = false;
        self.current_conversation_id = None; // Will create new conversation on next message
                                             // Reset session stats
        self.session_info = None;
        self.stats.cost = 0.0;
        self.stats.input_tokens = 0;
        self.stats.output_tokens = 0;
        // Reset active tasks and connection
        self.active_tasks.clear();
        self.connection_status = ConnectionStatus::Disconnected;
        // Reset think mode
        self.think_mode_enabled = false;
        cx.notify();
    }

    /// Get current conversation ID
    pub fn conversation_id(&self) -> Option<&str> {
        self.current_conversation_id.as_deref()
    }

    // ==================== Conversation Title ====================

    /// Get conversation title
    pub fn conversation_title(&self) -> Option<&str> {
        self.conversation_title.as_deref()
    }

    /// Set conversation title
    pub fn set_conversation_title(&mut self, title: Option<String>, cx: &mut Context<Self>) {
        self.conversation_title = title;
        cx.notify();
    }

    /// Get display title (title or auto-generated from first message)
    pub fn display_title(&self) -> String {
        if let Some(ref title) = self.conversation_title {
            title.clone()
        } else if let Some(first_user_msg) =
            self.messages.iter().find(|m| m.role == MessageRole::User)
        {
            // Auto-generate from first user message (truncated)
            let content = first_user_msg.content.trim();
            if content.len() > 50 {
                format!("{}...", &content[..47])
            } else {
                content.to_string()
            }
        } else {
            "New Conversation".to_string()
        }
    }
}
