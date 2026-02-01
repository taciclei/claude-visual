//! Onboarding tips and contextual recommendations

use gpui::*;

use super::core::ChatView;

impl ChatView {
    /// Dismiss an onboarding tip
    pub(crate) fn dismiss_tip(&mut self, tip_id: &'static str, cx: &mut Context<Self>) {
        self.dismissed_tips.insert(tip_id);
        cx.notify();
    }

    /// Dismiss an onboarding tip by string (for dynamically created tips)
    pub(crate) fn dismiss_tip_by_string(&mut self, tip_id: &str, cx: &mut Context<Self>) {
        // Map dynamic tip IDs to static ones
        let static_id = match tip_id {
            "tip-keyboard" => "tip-keyboard",
            "tip-commands" => "tip-commands",
            "tip-mention" => "tip-mention",
            "tip-think" => "tip-think",
            "tip-memory" => "tip-memory",
            "tip-mcp" => "tip-mcp",
            "tip-mcp-connected" => "tip-mcp-connected",
            "tip-resume" => "tip-resume",
            "tip-commit" => "tip-commit",
            "tip-review" => "tip-review",
            "tip-apex" => "tip-apex",
            _ => return, // Unknown tip, ignore
        };
        self.dismissed_tips.insert(static_id);
        cx.notify();
    }

    /// Check if a tip should be shown
    pub(crate) fn should_show_tip(&self, tip_id: &str) -> bool {
        self.panels.onboarding_tips && !self.dismissed_tips.contains(tip_id)
    }

    /// Get the recommended next workflow action based on current state
    /// Returns (icon, action_name, command, description)
    pub(crate) fn get_recommended_workflow_action(
        &self,
    ) -> Option<(&'static str, &'static str, &'static str, &'static str)> {
        // Priority 1: Context is critical
        if self.context_usage_percentage() > 0.85 {
            return Some((
                "🚨",
                "Compact Now",
                "/compact",
                "Context is critically full - compact immediately",
            ));
        }

        // Priority 2: Error recovery
        if self.last_error.is_some() {
            return Some(("🔄", "Retry", "retry", "Retry the last failed request"));
        }

        // Priority 3: Response was truncated
        if self.is_last_response_truncated() {
            return Some((
                "▶️",
                "Continue",
                "continue",
                "Continue the truncated response",
            ));
        }

        // Priority 4: Git workflow
        if let Some(ref git) = self.git_info {
            // Ready to commit
            if git.staged_count > 0 {
                return Some(("📦", "Commit", "/commit", "Commit your staged changes"));
            }
            // Changes need review
            if git.unstaged_count > 3 || git.untracked_count > 3 {
                return Some((
                    "👀",
                    "Review",
                    "/review",
                    "Review your changes before committing",
                ));
            }
            // Behind remote
            if git.behind > 5 {
                return Some((
                    "⬇️",
                    "Pull",
                    "git pull",
                    "Your branch is significantly behind remote",
                ));
            }
            // Ahead of remote
            if git.ahead > 3 && git.staged_count == 0 && git.unstaged_count == 0 {
                return Some((
                    "🔀",
                    "Push/PR",
                    "/create-pr",
                    "Push your commits or create a PR",
                ));
            }
        }

        // Priority 5: Context getting full
        if self.context_usage_percentage() > 0.7 {
            return Some((
                "🗜️",
                "Compact",
                "/compact",
                "Free up context space before it fills up",
            ));
        }

        // Priority 6: Long conversation without summary
        if self.messages.len() > 20 {
            return Some((
                "📋",
                "Summarize",
                "/summarize",
                "Summarize the conversation progress",
            ));
        }

        // No urgent action needed
        None
    }

    /// Get a random Claude Code pro tip based on conversation context
    pub(crate) fn get_contextual_pro_tip(
        &self,
    ) -> Option<(&'static str, &'static str, &'static str)> {
        // Context-aware tips: return specific tips based on current state

        // If context is getting full, prioritize compact tip
        if self.context_usage_percentage() > 0.75 {
            return Some((
                "🗜️",
                "Context Getting Full",
                "Use /compact now to free up context space before you run out",
            ));
        }

        // If there are git changes, suggest review/commit workflow
        if let Some(ref git) = self.git_info {
            if git.is_dirty && git.staged_count > 0 {
                return Some((
                    "📦",
                    "Ready to Commit",
                    "Use /commit to create a smart commit message for your staged changes",
                ));
            }
            if git.is_dirty && git.unstaged_count > 3 {
                return Some((
                    "👀",
                    "Time to Review",
                    "Use /review to get feedback on your code changes before committing",
                ));
            }
        }

        // If MCP servers are available but user might not know
        if let Some(ref info) = self.session_info {
            if !info.mcp_servers.is_empty() && self.messages.len() < 5 {
                return Some((
                    "🔌",
                    "MCP Connected",
                    "MCP servers ready. Press the MCP button or ⌘K to explore tools",
                ));
            }
        }

        // General pro tips organized by category: (icon, title, description)
        let tips: Vec<(&'static str, &'static str, &'static str)> = vec![
            // Implementation Skills
            (
                "⚡",
                "APEX Workflow",
                "Use /apex for full implementation workflow with validation",
            ),
            (
                "🚀",
                "Quick Implementation",
                "Use /oneshot for ultra-fast single-task implementation",
            ),
            (
                "🧠",
                "Deep Thinking",
                "Use /ultrathink for complex problems requiring careful reasoning",
            ),
            // Exploration Skills
            (
                "🔍",
                "Explore Codebase",
                "Use /explore to understand how code works",
            ),
            (
                "🔎",
                "Quick Search",
                "Use /search for lightning-fast answers to specific questions",
            ),
            (
                "📖",
                "Code Explanation",
                "Use /explain for deep code explanation with visual diagrams",
            ),
            (
                "📚",
                "Documentation",
                "Use /docs to research library documentation",
            ),
            // Code Quality Skills
            (
                "👀",
                "Expert Review",
                "Use /review-code for security, SOLID, and code smell analysis",
            ),
            (
                "♻️",
                "Refactoring",
                "Use /refactor for parallel code refactoring",
            ),
            (
                "✨",
                "Clean Code",
                "Use /clean-code to apply best practices automatically",
            ),
            (
                "🐛",
                "Debugging",
                "Use /debug for systematic error analysis and resolution",
            ),
            // Git & CI Skills
            (
                "📦",
                "Smart Commits",
                "Use /commit for conventional commit messages",
            ),
            (
                "🔀",
                "Pull Requests",
                "Use /create-pr to create PRs with auto-generated descriptions",
            ),
            (
                "💬",
                "PR Comments",
                "Use /fix-pr-comments to implement review feedback",
            ),
            (
                "🔧",
                "CI Fixer",
                "Use /ci-fixer to automatically fix CI/CD failures",
            ),
            // Research Skills
            (
                "💡",
                "Brainstorm",
                "Use /brainstorm for deep research with skeptical analysis",
            ),
            // Session Management
            (
                "💾",
                "Persistent Memory",
                "Use /memory to save important context to CLAUDE.md",
            ),
            (
                "📊",
                "Token Usage",
                "Use /usage to see detailed token consumption",
            ),
            (
                "🗜️",
                "Context Management",
                "Use /compact when context gets full",
            ),
            (
                "📜",
                "Session History",
                "Use /resume to continue previous conversations",
            ),
            // Tips for better usage
            (
                "🔍",
                "File Mentions",
                "Type @ followed by a filename to include files in context",
            ),
            (
                "⌨️",
                "Keyboard Shortcuts",
                "Press ⌘? to see all available keyboard shortcuts",
            ),
            (
                "🔌",
                "MCP Servers",
                "Connect external tools via MCP for extended capabilities",
            ),
            (
                "🔄",
                "Model Switching",
                "Press ⌘M to switch between Claude models (opus, sonnet, haiku)",
            ),
            (
                "🔀",
                "Parallel Agents",
                "Claude can launch parallel subagents for complex multi-file tasks",
            ),
            (
                "🎯",
                "Be Specific",
                "Include file paths and exact requirements for better results",
            ),
            // Advanced Skills
            (
                "🛠️",
                "Create Skills",
                "Use /create-skills to build custom workflow skills",
            ),
            (
                "🔗",
                "Create Hooks",
                "Use /create-hooks to automate Claude Code behaviors",
            ),
            (
                "📝",
                "Context Docs",
                "Use /create-context-docs to document for future sessions",
            ),
        ];

        // Select tip based on conversation state
        let msg_count = self.messages.len();
        if msg_count == 0 {
            return None; // Show welcome tips instead
        }

        // Use message count as pseudo-random seed for tip selection
        let tip_index = (msg_count * 7 + 3) % tips.len();
        Some(tips[tip_index])
    }
}
