//! Git command methods

use gpui::*;

use crate::ui::chat::view::types::*;
use crate::ui::chat::view::core::ChatView;

impl ChatView {
    // ==================== Git Commands ====================

    /// Send a Claude Code tool command
    pub fn send_tool_command(&mut self, tool: &str, args: Option<&str>, cx: &mut Context<Self>) {
        let cmd = if let Some(args) = args {
            format!("@{} {}", tool, args)
        } else {
            format!("@{}", tool)
        };
        cx.emit(ChatViewEvent::Submit(cmd));
    }

    /// Request code review
    pub fn request_code_review(&mut self, cx: &mut Context<Self>) {
        self.send_slash_command("/review", cx);
        self.show_notification("Requesting code review...", NotificationType::Info, cx);
    }

    /// Create a PR
    pub fn create_pr(&mut self, cx: &mut Context<Self>) {
        self.send_slash_command("/pr", cx);
        self.show_notification("Creating pull request...", NotificationType::Info, cx);
    }

    /// Show PR comments
    pub fn show_pr_comments(&mut self, cx: &mut Context<Self>) {
        self.send_slash_command("/pr-comments", cx);
    }

    /// Show status
    pub fn show_status(&mut self, cx: &mut Context<Self>) {
        self.send_slash_command("/status", cx);
    }
}
