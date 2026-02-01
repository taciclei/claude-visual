//! Slash command autocomplete

use super::utils::{fuzzy_match_commands, CommandMatch};
use super::ChatInput;
use gpui::*;

impl ChatInput {
    /// Set available slash commands (from Claude CLI session info)
    pub fn set_available_commands(&mut self, commands: Vec<String>, cx: &mut Context<Self>) {
        // Start with provided commands
        let mut all_commands = commands;

        // Add built-in Claude Code skills as fallback if not already present
        let builtin_skills = vec![
            // Core skills
            "apex",
            "brainstorm",
            "explore",
            "debug",
            "review",
            "oneshot",
            "explain",
            "refactor",
            "docs",
            "ultrathink",
            "search",
            // Git operations
            "commit",
            "create-pr",
            "fix-pr-comments",
            "merge",
            "pr",
            // CLI commands
            "think",
            "memory",
            "compact",
            "resume",
            "help",
            "usage",
            "add-dir",
            "status",
            "doctor",
            "mcp",
            "permissions",
            "config",
            "test",
            "summarize",
            "clear",
            "model",
            "vim",
            "init",
            "hooks",
            "allowed-tools",
            "task",
            "cost",
            "bug",
            "login",
            "logout",
            // Advanced skills
            "clean-code",
            "review-code",
            "ci-fixer",
            "keybindings-help",
            "create-hooks",
            "create-skills",
            "plan",
            "claude-memory",
            "create-prompt",
            "create-agent",
        ];

        for skill in builtin_skills {
            if !all_commands.iter().any(|c| c == skill) {
                all_commands.push(skill.to_string());
            }
        }

        // Sort alphabetically for better UX
        all_commands.sort();

        self.available_commands = all_commands;
        cx.notify();
    }

    /// Check if text starts with "/" and update autocomplete state with fuzzy matching
    pub(super) fn update_command_autocomplete(&mut self) {
        if self.text.starts_with('/') && !self.text.contains(' ') {
            // Extract the partial command (without the leading /)
            let partial = &self.text[1..];

            // Use fuzzy matching for better results
            let matches = fuzzy_match_commands(&self.available_commands, partial);

            // Store both commands and their match data
            self.command_matches = matches;
            self.filtered_commands = self
                .command_matches
                .iter()
                .map(|m| m.command.clone())
                .collect();

            // Show autocomplete if we have matches
            self.show_command_autocomplete = !self.filtered_commands.is_empty();
            self.selected_command_index = 0;
        } else {
            self.show_command_autocomplete = false;
            self.filtered_commands.clear();
            self.command_matches.clear();
        }
    }

    /// Get match data for a command at index
    pub fn get_command_match(&self, index: usize) -> Option<&CommandMatch> {
        self.command_matches.get(index)
    }

    /// Select previous command in autocomplete
    pub(super) fn select_previous_command(&mut self, cx: &mut Context<Self>) {
        if !self.filtered_commands.is_empty() {
            if self.selected_command_index > 0 {
                self.selected_command_index -= 1;
            } else {
                self.selected_command_index = self.filtered_commands.len() - 1;
            }
            cx.notify();
        }
    }

    /// Select next command in autocomplete
    pub(super) fn select_next_command(&mut self, cx: &mut Context<Self>) {
        if !self.filtered_commands.is_empty() {
            self.selected_command_index =
                (self.selected_command_index + 1) % self.filtered_commands.len();
            cx.notify();
        }
    }

    /// Insert selected command from autocomplete
    pub(super) fn insert_selected_command(&mut self, cx: &mut Context<Self>) {
        if let Some(cmd) = self
            .filtered_commands
            .get(self.selected_command_index)
            .cloned()
        {
            self.text = format!("/{} ", cmd);
            self.cursor_position = self.text.len();
            self.show_command_autocomplete = false;
            self.filtered_commands.clear();
            cx.notify();
        }
    }
}
