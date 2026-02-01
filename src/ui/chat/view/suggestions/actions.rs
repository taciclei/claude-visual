//! Quick action suggestions

use crate::claude::message::MessageRole;

use super::super::core::ChatView;

impl ChatView {
    /// Get quick action suggestions based on last response
    pub fn get_quick_actions(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        let mut actions = Vec::new();

        // If last response had code
        if let Some(last) = self
            .messages
            .iter()
            .rev()
            .find(|m| m.role == MessageRole::Assistant)
        {
            let content_lower = last.content.to_lowercase();

            if last.content.contains("```") {
                actions.push(("👀", "Review", "/review-code"));
                actions.push(("♻️", "Refactor", "/refactor"));
                actions.push(("🧪", "Test", "Write tests for this"));
                actions.push(("✨", "Clean", "/clean-code"));
            }

            // If response mentioned an error or debugging
            if content_lower.contains("error")
                || content_lower.contains("failed")
                || content_lower.contains("crash")
                || content_lower.contains("exception")
            {
                actions.push(("🐛", "Debug", "/debug"));
                actions.push(("🔧", "Fix", "Fix this error"));
                actions.push(("🔧", "CI Fix", "/ci-fixer"));
            }

            // If response was long or complex, offer deep think
            if last.content.len() > 2000 {
                actions.push(("🧠", "Think", "/ultrathink"));
                actions.push(("📋", "Summarize", "Summarize the key points"));
            }

            // If discussing architecture or design
            if content_lower.contains("architect")
                || content_lower.contains("design")
                || content_lower.contains("structure")
                || content_lower.contains("pattern")
            {
                actions.push(("💡", "Brainstorm", "/brainstorm"));
                actions.push(("🧠", "Analyze", "/ultrathink"));
            }

            // If response mentions commits, PR, or changes
            if content_lower.contains("commit")
                || content_lower.contains("changes")
                || content_lower.contains("pull request")
            {
                actions.push(("📦", "Commit", "/commit"));
                actions.push(("🔀", "Create PR", "/create-pr"));
                actions.push(("👀", "Review", "/review"));
            }

            // If discussing documentation
            if content_lower.contains("document")
                || content_lower.contains("readme")
                || content_lower.contains("api")
            {
                actions.push(("📖", "Explain", "/explain"));
                actions.push(("📚", "Docs", "/docs"));
            }

            // If discussing testing
            if content_lower.contains("test")
                || content_lower.contains("spec")
                || content_lower.contains("coverage")
            {
                actions.push(("🧪", "Tests", "Run these tests"));
                actions.push(("🐛", "Debug", "/debug"));
            }

            // If discussing CI/CD
            if content_lower.contains("ci")
                || content_lower.contains("pipeline")
                || content_lower.contains("build")
                || content_lower.contains("deploy")
            {
                actions.push(("🔧", "CI Fix", "/ci-fixer"));
                actions.push(("👁️", "Watch CI", "Monitor CI status"));
            }

            // If exploring or researching
            if content_lower.contains("found")
                || content_lower.contains("located")
                || content_lower.contains("search")
            {
                actions.push(("🔍", "Explore More", "/explore"));
                actions.push(("🔎", "Search", "/search"));
            }

            // If implementation discussion
            if content_lower.contains("implement")
                || content_lower.contains("create")
                || content_lower.contains("build")
                || content_lower.contains("add")
            {
                actions.push(("⚡", "APEX", "/apex"));
                actions.push(("🚀", "Oneshot", "/oneshot"));
            }
        }

        // Git-related actions when there are uncommitted changes
        if let Some(ref git) = self.git_info {
            if git.staged_count > 0 {
                if !actions.iter().any(|(_, label, _)| *label == "Commit") {
                    actions.push(("📦", "Commit", "/commit"));
                }
                actions.push(("🔀", "Create PR", "/create-pr"));
            }
            if git.is_dirty {
                actions.push(("👀", "Review", "/review"));
            }
            // If on a feature branch
            if !git.branch.is_empty() {
                if git.branch.starts_with("feature/") || git.branch.starts_with("fix/") {
                    actions.push(("🔀", "PR", "/create-pr"));
                }
            }
        }

        // If context is getting full - prioritize this
        let ctx_usage = self.context_usage_percentage();
        if ctx_usage > 0.85 {
            actions.insert(0, ("🗜️", "Compact!", "/compact"));
            actions.push(("📝", "Memory", "/memory"));
        } else if ctx_usage > 0.6 {
            actions.push(("🗜️", "Compact", "/compact"));
        }

        // If there's an error
        if self.last_error.is_some() {
            actions.insert(0, ("🐛", "Debug", "/debug"));
            actions.insert(1, ("🔄", "Retry", "retry"));
        }

        // If truncated response
        if self.is_last_response_truncated() {
            actions.insert(0, ("➡️", "Continue", "Continue"));
        }

        // If no actions yet, suggest Claude Code skills based on session state
        if actions.is_empty() {
            if self.messages.is_empty() {
                // Fresh session - exploration focused
                actions.push(("🔍", "Explore", "/explore"));
                actions.push(("📖", "Explain", "/explain"));
                actions.push(("🔎", "Search", "/search"));
            } else {
                // Active session - implementation focused
                actions.push(("⚡", "APEX", "/apex"));
                actions.push(("🚀", "Oneshot", "/oneshot"));
                actions.push(("🧠", "Think", "/ultrathink"));
            }
        }

        // Remove duplicates based on label
        let mut seen = std::collections::HashSet::new();
        actions.retain(|(_, label, _)| seen.insert(*label));

        actions.truncate(6);
        actions
    }

    /// Get skill suggestions based on current tool being used
    pub fn get_tool_based_suggestions(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        let tool = match &self.current_tool_name {
            Some(t) => t.to_lowercase(),
            None => return vec![],
        };

        match tool.as_str() {
            "read" => vec![
                ("📖", "Explain", "/explain"),
                ("♻️", "Refactor", "/refactor"),
                ("👀", "Review", "/review-code"),
            ],
            "write" | "edit" => vec![
                ("👀", "Review", "/review-code"),
                ("✨", "Clean", "/clean-code"),
                ("🧪", "Test", "Write tests"),
            ],
            "bash" => vec![
                ("🐛", "Debug", "/debug"),
                ("🔧", "CI Fix", "/ci-fixer"),
                ("📖", "Explain", "Explain this command"),
            ],
            "grep" | "glob" => vec![("🔍", "Explore", "/explore"), ("📖", "Explain", "/explain")],
            "task" => vec![("⚡", "APEX", "/apex"), ("🧠", "Think", "/ultrathink")],
            "websearch" | "webfetch" => {
                vec![("💡", "Brainstorm", "/brainstorm"), ("📚", "Docs", "/docs")]
            }
            _ => vec![],
        }
    }
}
