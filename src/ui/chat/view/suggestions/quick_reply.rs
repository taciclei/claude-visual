//! Quick reply suggestion generation

use gpui::*;

use crate::claude::message::MessageRole;

use super::super::core::ChatView;
use super::types::QuickReplySuggestion;

impl ChatView {
    /// Generate quick reply suggestions based on the last assistant message
    pub fn generate_quick_reply_suggestions(&mut self, cx: &mut Context<Self>) {
        self.quick_reply_suggestions.clear();

        // Get the last assistant message
        let last_assistant = self.messages.iter().rev()
            .find(|m| m.role == MessageRole::Assistant)
            .map(|m| m.content.clone());

        if let Some(content) = last_assistant {
            // Add contextual suggestions based on content
            let content_lower = content.to_lowercase();

            // Add common follow-ups
            if content.len() > 500 {
                self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                    "Summarize",
                    "Can you provide a brief summary of what you just explained?",
                    "action",
                    "📋"
                ));
            }

            if content_lower.contains("example") || content_lower.contains("code") {
                self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                    "Different example",
                    "Can you show me a different example?",
                    "explore",
                    "🔀"
                ));
            }

            if content_lower.contains("error") || content_lower.contains("bug") || content_lower.contains("fix") {
                self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                    "Explain fix",
                    "Can you explain why this fix works?",
                    "clarify",
                    "💡"
                ));
                self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                    "Prevent future",
                    "How can I prevent this issue in the future?",
                    "explore",
                    "🛡️"
                ));
            }

            if content_lower.contains("function") || content_lower.contains("method") || content_lower.contains("class") {
                self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                    "Add tests",
                    "Can you add unit tests for this?",
                    "action",
                    "🧪"
                ));
            }

            // Code block related suggestions
            if content.contains("```") {
                self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                    "Improve code",
                    "Can you improve this code for better readability and performance?",
                    "action",
                    "✨"
                ));
                if !content_lower.contains("comment") {
                    self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                        "Add comments",
                        "Can you add helpful comments to this code?",
                        "action",
                        "📝"
                    ));
                }
            }

            // Performance related
            if content_lower.contains("performance") || content_lower.contains("optimize") || content_lower.contains("slow") {
                self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                    "Benchmark",
                    "How can I benchmark this to measure the improvement?",
                    "explore",
                    "⏱️"
                ));
            }

            // Security related
            if content_lower.contains("security") || content_lower.contains("vulnerability") || content_lower.contains("auth") {
                self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                    "Security audit",
                    "Are there any other security considerations I should be aware of?",
                    "explore",
                    "🔒"
                ));
            }

            // API/Integration related
            if content_lower.contains("api") || content_lower.contains("endpoint") || content_lower.contains("request") {
                self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                    "Error handling",
                    "How should I handle errors for this API call?",
                    "explore",
                    "⚠️"
                ));
            }

            // Claude Code workflow suggestions based on content
            if content_lower.contains("implement") || content_lower.contains("create") || content_lower.contains("build") {
                self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                    "APEX it",
                    "/apex Continue with full implementation workflow",
                    "command",
                    "⚡"
                ));
                self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                    "Think deeper",
                    "/ultrathink Analyze this more carefully before implementing",
                    "command",
                    "🧠"
                ));
            }

            if content_lower.contains("change") || content_lower.contains("modif") || content_lower.contains("update") {
                self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                    "Review changes",
                    "/review Please review what we've changed so far",
                    "command",
                    "👀"
                ));
                self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                    "Refactor",
                    "/refactor Improve the code structure",
                    "command",
                    "♻️"
                ));
            }

            // Exploration suggestions
            if content_lower.contains("understand") || content_lower.contains("how does") || content_lower.contains("what is") {
                self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                    "Explore deeper",
                    "/explore Explore this part of the codebase more thoroughly",
                    "command",
                    "🔍"
                ));
            }

            // Git-related suggestions
            if let Some(ref git) = self.git_info {
                if git.is_dirty && (content_lower.contains("done") || content_lower.contains("complete") || content_lower.contains("finish")) {
                    self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                        "Commit",
                        "/commit Create a commit for these changes",
                        "command",
                        "📦"
                    ));
                    self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                        "Create PR",
                        "/create-pr Create a pull request",
                        "command",
                        "🔀"
                    ));
                }
            }

            // If discussing architecture or design
            if content_lower.contains("architect") || content_lower.contains("design") || content_lower.contains("structure") {
                self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                    "Brainstorm",
                    "/brainstorm Research and analyze this design more deeply",
                    "command",
                    "💡"
                ));
            }

            // Debug suggestions
            if content_lower.contains("fail") || content_lower.contains("broken") || content_lower.contains("doesn't work") {
                self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                    "Debug",
                    "/debug Systematically debug this issue",
                    "command",
                    "🐛"
                ));
            }

            // CI/CD suggestions
            if content_lower.contains("ci") || content_lower.contains("pipeline") || content_lower.contains("build fail") {
                self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                    "Fix CI",
                    "/ci-fixer Automatically fix CI failures",
                    "command",
                    "🔧"
                ));
            }

            // Documentation suggestions
            if content_lower.contains("document") || content_lower.contains("readme") || content_lower.contains("explain") {
                self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                    "Research docs",
                    "/docs Research the documentation",
                    "command",
                    "📚"
                ));
            }

            // Always add some generic follow-ups if we don't have many
            if self.quick_reply_suggestions.len() < 3 {
                self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                    "Explain more",
                    "Can you explain this in more detail?",
                    "clarify",
                    "💡"
                ));
                self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                    "Alternative",
                    "What's an alternative approach?",
                    "explore",
                    "🔀"
                ));
            }

            // Add Claude Code workflow suggestion as last option
            if self.quick_reply_suggestions.len() < 4 {
                self.quick_reply_suggestions.push(QuickReplySuggestion::new(
                    "Save to memory",
                    "/memory Remember this context for future sessions",
                    "command",
                    "💾"
                ));
            }

            // Limit to 4 suggestions
            self.quick_reply_suggestions.truncate(4);
        }

        cx.notify();
    }
}
