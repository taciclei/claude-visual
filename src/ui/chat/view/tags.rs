//! Tag management functionality for ChatView

use gpui::*;
use super::ChatView;

impl ChatView {
    /// Toggle tags editor
    pub fn toggle_tags_editor(&mut self, cx: &mut Context<Self>) {
        self.panels.tags_editor = !self.panels.tags_editor;
        cx.notify();
    }

    /// Add a tag to the conversation
    pub fn add_tag(&mut self, tag: impl Into<String>, cx: &mut Context<Self>) {
        let tag = tag.into();
        if !self.conversation_tags.contains(&tag) {
            self.conversation_tags.push(tag);
            cx.notify();
        }
    }

    /// Remove a tag from the conversation
    pub fn remove_tag(&mut self, tag: &str, cx: &mut Context<Self>) {
        self.conversation_tags.retain(|t| t != tag);
        cx.notify();
    }

    /// Get conversation tags
    pub fn get_tags(&self) -> &[String] {
        &self.conversation_tags
    }

    /// Check if conversation has tags
    pub fn has_tags(&self) -> bool {
        !self.conversation_tags.is_empty()
    }

    /// Suggested tags based on conversation content
    pub fn suggest_tags(&self) -> Vec<&'static str> {
        let mut suggestions = Vec::new();

        // Analyze messages to suggest tags
        let content: String = self.messages.iter()
            .map(|m| m.content.to_lowercase())
            .collect::<Vec<_>>()
            .join(" ");

        if content.contains("bug") || content.contains("fix") || content.contains("error") {
            suggestions.push("bugfix");
        }
        if content.contains("feature") || content.contains("implement") || content.contains("add") {
            suggestions.push("feature");
        }
        if content.contains("refactor") || content.contains("clean") || content.contains("improve") {
            suggestions.push("refactor");
        }
        if content.contains("test") || content.contains("spec") {
            suggestions.push("testing");
        }
        if content.contains("doc") || content.contains("readme") || content.contains("comment") {
            suggestions.push("documentation");
        }
        if content.contains("review") || content.contains("pr") || content.contains("pull request") {
            suggestions.push("code-review");
        }
        if content.contains("deploy") || content.contains("ci") || content.contains("cd") {
            suggestions.push("devops");
        }
        if content.contains("debug") || content.contains("investigate") {
            suggestions.push("debugging");
        }
        if content.contains("design") || content.contains("architect") || content.contains("pattern") {
            suggestions.push("architecture");
        }
        if content.contains("learn") || content.contains("explain") || content.contains("understand") {
            suggestions.push("learning");
        }

        suggestions
    }
}
