//! Contextual suggestion updates and management

use gpui::*;

use super::super::core::ChatView;
use super::super::types::ConnectionStatus;
use super::super::ChatViewEvent;
use super::types::ContextualSuggestion;

impl ChatView {
    /// Update contextual suggestions based on current state
    pub fn update_suggestions(&mut self, cx: &mut Context<Self>) {
        self.contextual_suggestions.clear();

        // Suggestions based on conversation state
        if self.messages.is_empty() {
            // Empty conversation - suggest Claude Code skills
            self.contextual_suggestions.push(ContextualSuggestion::new(
                "/explore - Explore this codebase",
                "🔍",
                "skill",
                100,
            ));
            self.contextual_suggestions.push(ContextualSuggestion::new(
                "/apex - Full APEX workflow",
                "⚡",
                "skill",
                95,
            ));
            self.contextual_suggestions.push(ContextualSuggestion::new(
                "/ultrathink - Deep analysis",
                "🧠",
                "skill",
                90,
            ));
            self.contextual_suggestions.push(ContextualSuggestion::new(
                "/brainstorm - Research any topic",
                "💡",
                "skill",
                85,
            ));
            self.contextual_suggestions.push(ContextualSuggestion::new(
                "/search - Quick answer search",
                "⚡",
                "skill",
                80,
            ));
            self.contextual_suggestions.push(ContextualSuggestion::new(
                "/review - Code review",
                "👀",
                "skill",
                75,
            ));
        } else {
            // Based on last message content
            if let Some(last_msg) = self.messages.last() {
                let content_lower = last_msg.content.to_lowercase();

                // If there was an error, suggest retry or fix
                if last_msg.is_error || content_lower.contains("error") {
                    self.contextual_suggestions.push(ContextualSuggestion::new(
                        "Can you fix this error?",
                        "🔧",
                        "fix",
                        100,
                    ));
                    self.contextual_suggestions.push(ContextualSuggestion::new(
                        "Explain what went wrong",
                        "❓",
                        "explain",
                        90,
                    ));
                }

                // If code was shown, suggest actions
                if content_lower.contains("```") {
                    self.contextual_suggestions.push(ContextualSuggestion::new(
                        "Add comments to this code",
                        "💬",
                        "code",
                        80,
                    ));
                    self.contextual_suggestions.push(ContextualSuggestion::new(
                        "Write tests for this",
                        "🧪",
                        "test",
                        75,
                    ));
                    self.contextual_suggestions.push(ContextualSuggestion::new(
                        "Refactor this code",
                        "♻️",
                        "refactor",
                        70,
                    ));
                }

                // If it was a long response, suggest summary
                if last_msg.content.len() > 2000 {
                    self.contextual_suggestions.push(ContextualSuggestion::new(
                        "Summarize the key points",
                        "📝",
                        "summarize",
                        85,
                    ));
                }
            }

            // Context-based suggestions
            if self.context_usage_percentage() > 0.7 {
                self.contextual_suggestions.push(ContextualSuggestion::new(
                    "/compact - Free up context space",
                    "📦",
                    "command",
                    95,
                ));
            }
        }

        // Git-related suggestions based on current status
        if let Some(ref git) = self.git_info {
            if git.is_dirty {
                if git.staged_count > 0 {
                    self.contextual_suggestions.push(ContextualSuggestion::new(
                        "/commit - Commit staged changes",
                        "✅",
                        "git",
                        88,
                    ));
                }
                if git.unstaged_count > 0 || git.untracked_count > 0 {
                    self.contextual_suggestions.push(ContextualSuggestion::new(
                        "Review my changes",
                        "👀",
                        "review",
                        82,
                    ));
                }
            }
            if git.behind > 0 {
                self.contextual_suggestions.push(ContextualSuggestion::new(
                    "Pull latest changes",
                    "⬇️",
                    "git",
                    85,
                ));
            }
        }

        // Session-based skill suggestions (when no git actions and session available)
        if self.contextual_suggestions.len() < 3 {
            if let Some(ref info) = self.session_info {
                // Suggest explore agent if available
                if info
                    .agents
                    .iter()
                    .any(|a| a.contains("explore") || a.contains("Explore"))
                {
                    self.contextual_suggestions.push(ContextualSuggestion::new(
                        "Explore and understand this codebase",
                        "🔍",
                        "explore",
                        75,
                    ));
                }
                // Suggest code review if available
                if info.skills.iter().any(|s| s.contains("review")) {
                    self.contextual_suggestions.push(ContextualSuggestion::new(
                        "/review - Review code quality",
                        "👁️",
                        "skill",
                        72,
                    ));
                }
                // Suggest test writing if available
                if info.skills.iter().any(|s| s.contains("test")) {
                    self.contextual_suggestions.push(ContextualSuggestion::new(
                        "Write comprehensive tests",
                        "✅",
                        "test",
                        70,
                    ));
                }
            }
        }

        // Claude Code advanced features suggestions
        if let Some(last_msg) = self.messages.last() {
            let content_lower = last_msg.content.to_lowercase();

            // Suggest /think for complex analysis or architecture discussions
            if content_lower.contains("architect")
                || content_lower.contains("design")
                || content_lower.contains("complex")
                || content_lower.contains("strategy")
            {
                self.contextual_suggestions.push(ContextualSuggestion::new(
                    "/think - Deep analysis mode",
                    "🧠",
                    "command",
                    88,
                ));
            }

            // Suggest /memory when discussing important context
            if content_lower.contains("remember")
                || content_lower.contains("important")
                || content_lower.contains("always")
                || content_lower.contains("note that")
            {
                self.contextual_suggestions.push(ContextualSuggestion::new(
                    "/memory - Save to memory",
                    "💾",
                    "command",
                    86,
                ));
            }

            // Suggest /debug for troubleshooting
            if content_lower.contains("doesn't work")
                || content_lower.contains("not working")
                || content_lower.contains("broken")
                || content_lower.contains("crash")
            {
                self.contextual_suggestions.push(ContextualSuggestion::new(
                    "/debug - Debug this issue",
                    "🐛",
                    "command",
                    92,
                ));
            }

            // Suggest /test for code implementation discussions
            if content_lower.contains("implement")
                || content_lower.contains("function")
                || content_lower.contains("method")
                || content_lower.contains("class")
            {
                self.contextual_suggestions.push(ContextualSuggestion::new(
                    "Write tests for this",
                    "🧪",
                    "test",
                    78,
                ));
            }

            // Suggest /docs for API or interface discussions
            if content_lower.contains("api")
                || content_lower.contains("interface")
                || content_lower.contains("documentation")
                || content_lower.contains("readme")
            {
                self.contextual_suggestions.push(ContextualSuggestion::new(
                    "Generate documentation",
                    "📖",
                    "docs",
                    76,
                ));
            }

            // Suggest /refactor for code quality discussions
            if content_lower.contains("ugly")
                || content_lower.contains("messy")
                || content_lower.contains("clean up")
                || content_lower.contains("improve")
            {
                self.contextual_suggestions.push(ContextualSuggestion::new(
                    "Refactor this code",
                    "♻️",
                    "refactor",
                    80,
                ));
            }
        }

        // Smart workflow suggestions based on session state
        if self.messages.len() > 10 && !self.streaming.is_streaming {
            // After many messages, suggest summarization
            self.contextual_suggestions.push(ContextualSuggestion::new(
                "Summarize our progress",
                "📋",
                "summarize",
                65,
            ));
        }

        // Suggest /resume if disconnected and has recent sessions
        if matches!(self.connection_status, ConnectionStatus::Disconnected)
            && !self.recent_sessions.is_empty()
        {
            self.contextual_suggestions.push(ContextualSuggestion::new(
                "/resume - Resume previous session",
                "▶️",
                "command",
                90,
            ));
        }

        // Sort by priority (highest first)
        self.contextual_suggestions
            .sort_by(|a, b| b.priority.cmp(&a.priority));

        // Keep only top 4
        self.contextual_suggestions.truncate(4);

        cx.notify();
    }

    /// Toggle suggestions visibility
    pub fn toggle_suggestions(&mut self, cx: &mut Context<Self>) {
        self.panels.suggestions = !self.panels.suggestions;
        cx.notify();
    }

    /// Use a suggestion (sends it as a prompt)
    pub fn use_suggestion(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(suggestion) = self.contextual_suggestions.get(index) {
            let text = suggestion.text.clone();
            cx.emit(ChatViewEvent::Submit(text));
        }
    }
}
