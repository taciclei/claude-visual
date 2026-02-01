//! Smart command suggestions and contextual placeholders

use crate::claude::message::MessageRole;

use super::super::core::ChatView;

impl ChatView {
    /// Get smart command suggestions based on current context
    pub fn get_smart_commands(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        let mut commands: Vec<(&'static str, &'static str, &'static str)> = Vec::new();

        // Context-based urgent suggestions first
        if self.context_usage_percentage() > 0.6 {
            commands.push(("/compact", "⚠️ Context filling up - compact now", "📦"));
        }

        // Session-based suggestions
        if self.last_error.is_some() {
            commands.push(("/debug", "Systematic error debugging", "🐛"));
            commands.push(("/doctor", "Run diagnostics to fix issues", "🩺"));
        }

        // Git-based suggestions (high priority when changes exist)
        if let Some(ref git) = self.git_info {
            if git.staged_count > 0 {
                commands.push(("/commit", "Commit staged changes", "📦"));
            }
            if git.is_dirty {
                commands.push(("/review", "Review code changes", "👀"));
                if git.unstaged_count > 3 {
                    commands.push(("/refactor", "Refactor code with parallel agents", "♻️"));
                }
            }
            if git.staged_count > 0 || git.is_dirty {
                commands.push(("/create-pr", "Create pull request", "🔀"));
            }
        }

        // Claude Code skill suggestions based on conversation state
        let message_count = self.messages.len();

        if message_count == 0 {
            // Fresh conversation - suggest exploration
            commands.push(("/explore", "Explore codebase", "🔍"));
            commands.push(("/search", "Fast search for answers", "⚡"));
        } else if message_count < 5 {
            // Early conversation - suggest structured workflows
            commands.push(("/apex", "Full APEX workflow", "⚡"));
            commands.push(("/ultrathink", "Deep thinking mode", "🧠"));
        }

        // Check last message content for context-aware suggestions
        if let Some(last) = self
            .messages
            .iter()
            .rev()
            .find(|m| m.role == MessageRole::Assistant)
        {
            let content_lower = last.content.to_lowercase();

            if content_lower.contains("error") || content_lower.contains("bug") {
                commands.push(("/debug", "Debug the error", "🐛"));
            }
            if content_lower.contains("```") {
                commands.push(("/explain", "Explain this code", "📖"));
                commands.push(("/clean-code", "Apply clean code principles", "✨"));
            }
            if content_lower.contains("test") {
                commands.push(("/ci-fixer", "Fix CI issues", "🔧"));
            }
            if content_lower.contains("design") || content_lower.contains("architect") {
                commands.push(("/brainstorm", "Deep research", "💡"));
            }
        }

        // Always useful commands
        commands.push(("/help", "Show available commands", "❓"));
        commands.push(("/usage", "Show token usage", "📊"));
        commands.push(("/think", "Enable extended thinking", "🧠"));
        commands.push(("/memory", "Manage persistent memory", "💾"));

        // Remove duplicates and limit
        let mut seen = std::collections::HashSet::new();
        commands.retain(|(cmd, _, _)| seen.insert(*cmd));
        commands.truncate(10);
        commands
    }

    /// Get contextual placeholder text based on conversation state
    pub fn get_contextual_placeholder(&self) -> &'static str {
        // If there's an error, suggest retry
        if self.last_error.is_some() {
            return "Press ⌘R to retry, or type a new message...";
        }

        // If streaming, show waiting message
        if self.streaming.is_streaming {
            return "Claude is thinking...";
        }

        // If response appears truncated
        if self.is_last_response_truncated() {
            return "Type 'continue' to get more, or ask a follow-up...";
        }

        // If context is critical
        if self.context_usage_percentage() > 0.85 {
            return "⚠️ Context almost full - use /compact NOW to free space";
        }

        // If context is running low
        if self.context_usage_percentage() > 0.7 {
            return "Context filling up - consider /compact soon...";
        }

        // Based on conversation length
        let message_count = self.messages.len();

        if message_count == 0 {
            // Empty conversation - suggest starting points
            return "Ask anything... Type / for skills, @ for files";
        }

        // Git-aware suggestions
        if let Some(ref git) = self.git_info {
            if git.staged_count > 0 {
                return "Use /commit to commit staged changes, or continue...";
            }
            if git.is_dirty && git.unstaged_count > 5 {
                return "Many changes pending - use /review or continue...";
            }
        }

        // Check if last message was code-related
        if let Some(last) = self
            .messages
            .iter()
            .rev()
            .find(|m| m.role == MessageRole::Assistant)
        {
            if last.content.contains("```") {
                return "Ask to explain, test, or improve the code...";
            }
            // Check for architectural discussions
            let content_lower = last.content.to_lowercase();
            if content_lower.contains("architect") || content_lower.contains("design") {
                return "Use /think for deeper analysis, or continue...";
            }
        }

        // Default suggestions based on conversation stage
        if message_count < 4 {
            "Follow up with details... Try /think for complex problems"
        } else if message_count < 10 {
            "Continue or try /apex for structured implementation..."
        } else if message_count < 20 {
            "Keep going... Use /summarize if getting long"
        } else {
            "Long conversation - consider /compact or /summarize"
        }
    }
}
