//! Title and summary related functionality for ChatView

use gpui::*;

use crate::claude::message::MessageRole;
use super::core::ChatView;
use super::types::{ChatViewEvent, NotificationType};

impl ChatView {
    // ==================== Auto-Title & Summary ====================

    /// Generate an auto-title from the first user message
    pub fn auto_generate_title(&mut self, cx: &mut Context<Self>) {
        if self.conversation_title.is_some() {
            return; // Already has a title
        }

        // Find first user message
        if let Some(first_user_msg) = self.messages.iter().find(|m| m.role == MessageRole::User) {
            let content = &first_user_msg.content;

            // Generate title from first line or first N characters
            let title = if let Some(first_line) = content.lines().next() {
                let line = first_line.trim();
                if line.len() <= 50 {
                    line.to_string()
                } else {
                    format!("{}...", &line[..47])
                }
            } else if content.len() <= 50 {
                content.trim().to_string()
            } else {
                format!("{}...", &content[..47])
            };

            if !title.is_empty() {
                self.conversation_title = Some(title);
                cx.notify();
            }
        }
    }

    /// Request Claude to generate a better title (sends a prompt)
    pub fn request_ai_title(&mut self, cx: &mut Context<Self>) {
        if self.messages.is_empty() {
            self.show_notification("No messages to summarize", NotificationType::Warning, cx);
            return;
        }

        // Build a summary request
        let prompt = "Based on this conversation so far, suggest a short, descriptive title (max 50 chars). Reply with just the title, nothing else.".to_string();
        cx.emit(ChatViewEvent::Submit(prompt));
        self.show_notification("Requesting AI title suggestion...", NotificationType::Info, cx);
    }

    /// Request a conversation summary
    pub fn request_summary(&mut self, cx: &mut Context<Self>) {
        if self.messages.is_empty() {
            self.show_notification("No messages to summarize", NotificationType::Warning, cx);
            return;
        }

        let prompt = "Please provide a brief summary of what we've discussed and accomplished in this conversation so far. Include key decisions made and any pending items.".to_string();
        cx.emit(ChatViewEvent::Submit(prompt));
        self.show_notification("Requesting conversation summary...", NotificationType::Info, cx);
    }

    /// Get a quick conversation summary (local, no AI call)
    pub fn get_quick_summary(&self) -> String {
        let user_count = self.messages.iter().filter(|m| m.role == MessageRole::User).count();
        let assistant_count = self.messages.iter().filter(|m| m.role == MessageRole::Assistant).count();
        let tool_count = self.messages.iter().filter(|m| matches!(m.role, MessageRole::ToolUse | MessageRole::ToolResult)).count();

        let total_words: usize = self.messages.iter()
            .map(|m| m.content.split_whitespace().count())
            .sum();

        format!(
            "{} messages ({} you, {} Claude, {} tools), ~{} words",
            self.messages.len(), user_count, assistant_count, tool_count, total_words
        )
    }

    /// Export conversation as a shareable summary
    pub fn export_shareable_summary(&self) -> String {
        let mut summary = String::new();

        // Header
        if let Some(ref title) = self.conversation_title {
            summary.push_str(&format!("# {}\n\n", title));
        } else {
            summary.push_str("# Conversation Summary\n\n");
        }

        // Stats
        summary.push_str(&format!("**Summary:** {}\n\n", self.get_quick_summary()));

        // Key highlights (first and last few messages)
        summary.push_str("## Key Messages\n\n");

        for (i, msg) in self.messages.iter().enumerate() {
            if i < 2 || i >= self.messages.len().saturating_sub(2) {
                let role = match msg.role {
                    MessageRole::User => "You",
                    MessageRole::Assistant => "Claude",
                    _ => continue,
                };
                let preview = if msg.content.len() > 200 {
                    format!("{}...", &msg.content[..200])
                } else {
                    msg.content.clone()
                };
                summary.push_str(&format!("**{}:** {}\n\n", role, preview));
            }
            if i == 2 && self.messages.len() > 4 {
                summary.push_str("*... (messages omitted) ...*\n\n");
            }
        }

        summary
    }

    /// Extract file paths mentioned in the conversation
    pub fn extract_mentioned_files(&self) -> Vec<String> {
        let mut files = Vec::new();

        // Simple pattern matching for file paths (without regex)
        for msg in &self.messages {
            // Look for common file patterns
            for word in msg.content.split_whitespace() {
                let trimmed = word.trim_matches(|c: char| c == '`' || c == '"' || c == '\'' || c == ':' || c == ',');
                // Check if it looks like a file path
                if (trimmed.contains('/') || trimmed.starts_with('.'))
                    && trimmed.contains('.')
                    && !trimmed.starts_with("http")
                    && !trimmed.contains("...")
                    && trimmed.len() > 3
                    && trimmed.len() < 200
                {
                    let path = trimmed.to_string();
                    if !files.contains(&path) {
                        files.push(path);
                    }
                }
            }
        }

        // Limit to most recent 20
        if files.len() > 20 {
            files = files.into_iter().rev().take(20).collect();
            files.reverse();
        }

        files
    }

    /// Quick mention common project files
    pub fn quick_mention_readme(&mut self, cx: &mut Context<Self>) {
        self.insert_file_mention("README.md", cx);
    }

    pub fn quick_mention_package(&mut self, cx: &mut Context<Self>) {
        self.insert_file_mention("package.json", cx);
    }

    pub fn quick_mention_cargo(&mut self, cx: &mut Context<Self>) {
        self.insert_file_mention("Cargo.toml", cx);
    }
}
