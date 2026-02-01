//! Command palette execution logic
//!
//! This module contains the command execution logic that maps command IDs
//! to their corresponding actions in the ChatView.

use super::super::core::ChatView;
use super::super::types::{MessageFilter, NotificationType};
use gpui::*;

impl ChatView {
    /// Execute a command from the palette
    pub fn execute_palette_command(&mut self, command_id: &str, cx: &mut Context<Self>) {
        self.panels.command_palette = false;
        self.palette.query.clear();

        match command_id {
            // Navigation
            "scroll_to_top" => self.select_first_message(cx),
            "scroll_to_bottom" => self.select_last_message(cx),
            "prev_message" => self.select_prev_message(cx),
            "next_message" => self.select_next_message(cx),
            "navigate_back" => self.navigate_back(cx),
            "navigate_forward" => self.navigate_forward(cx),
            "jump_to_search_result" => self.scroll_to_search_result(cx),

            // View
            "toggle_search" => self.toggle_search(cx),
            "toggle_stats" => self.toggle_stats(cx),
            "toggle_timestamps" => self.toggle_timestamps(cx),
            "toggle_time_separators" => self.toggle_time_separators(cx),
            "toggle_compact" => self.toggle_compact_mode(cx),
            "toggle_word_wrap" => self.toggle_word_wrap(cx),
            "toggle_line_numbers" => self.toggle_line_numbers(cx),
            "toggle_theme" => self.request_theme_toggle(cx),

            // Actions
            "copy_conversation" => self.copy_conversation_to_clipboard(cx),
            "export_conversation" => self.request_export(cx),
            "clear_conversation" => self.request_clear_conversation(cx),
            "copy_selected" => self.copy_selected_message(cx),
            "bookmark_selected" => self.bookmark_selected_message(cx),
            "edit_last_message" => self.edit_last_message(cx),
            "pin_selected" => {
                if let Some(idx) = self.selected_message_index {
                    self.toggle_pin(idx, cx);
                }
            }
            "copy_last_response" => self.copy_last_response(cx),

            // Message Actions (on selected)
            "branch_selected" => {
                if let Some(idx) = self.selected_message_index {
                    self.branch_from_message(idx, cx);
                } else {
                    self.show_notification("No message selected", NotificationType::Warning, cx);
                }
            }
            "retry_selected" => {
                if let Some(idx) = self.selected_message_index {
                    self.retry_from_message(idx, cx);
                } else {
                    self.show_notification("No message selected", NotificationType::Warning, cx);
                }
            }
            "quote_selected" => {
                if let Some(idx) = self.selected_message_index {
                    self.quote_message(idx, cx);
                } else {
                    self.show_notification("No message selected", NotificationType::Warning, cx);
                }
            }
            "delete_selected" => {
                if let Some(idx) = self.selected_message_index {
                    self.delete_message_at(idx, cx);
                } else {
                    self.show_notification("No message selected", NotificationType::Warning, cx);
                }
            }

            // Messages
            "expand_all" => self.expand_all(cx),
            "collapse_all" => self.collapse_all(cx),
            "collapse_tools" => self.collapse_tool_messages(cx),
            "expand_tools" => self.expand_tool_messages(cx),
            "toggle_tools" => self.toggle_collapse_tool_messages(cx),
            "collapse_assistant" => self.collapse_assistant_messages(cx),
            "expand_assistant" => self.expand_assistant_messages(cx),
            "filter_all" => self.set_message_filter(MessageFilter::All, cx),
            "filter_user" => self.set_message_filter(MessageFilter::UserOnly, cx),
            "filter_assistant" => self.set_message_filter(MessageFilter::AssistantOnly, cx),
            "filter_tools" => self.set_message_filter(MessageFilter::ToolsOnly, cx),

            // Claude CLI Commands (send as text)
            "cmd_resume" => self.send_slash_command("/resume", cx),
            "cmd_usage" => self.send_slash_command("/usage", cx),
            "cmd_help" => self.send_slash_command("/help", cx),
            "cmd_config" => self.send_slash_command("/config", cx),
            "cmd_memory" => self.send_slash_command("/memory", cx),
            "cmd_model" => self.send_slash_command("/model", cx),
            "cmd_compact" => self.send_slash_command("/compact", cx),
            "cmd_vim" => self.send_slash_command("/vim", cx),
            "cmd_doctor" => self.send_slash_command("/doctor", cx),
            "cmd_permissions" => self.send_slash_command("/permissions", cx),
            "cmd_init" => self.send_slash_command("/init", cx),
            "cmd_add_dir" => self.send_slash_command("/add-dir", cx),
            "cmd_clear" => self.send_slash_command("/clear", cx),
            "cmd_cost" => self.show_cost(cx),
            "cmd_status" => self.show_status(cx),
            "cmd_think" => self.enable_think_mode(cx),
            "cmd_think_off" => self.disable_think_mode(cx),
            "toggle_think" => self.toggle_think_mode(cx),
            "cmd_review" => self.request_code_review(cx),
            "cmd_pr" => self.create_pr(cx),
            "cmd_pr_comments" => self.show_pr_comments(cx),
            "cmd_login" => self.send_slash_command("/login", cx),
            "cmd_logout" => self.send_slash_command("/logout", cx),
            "cmd_bug" => self.send_slash_command("/bug", cx),
            "cmd_mcp" => self.send_slash_command("/mcp", cx),

            // Response Actions
            "continue_response" => self.continue_conversation(cx),
            "regenerate_response" => self.regenerate_last_response(cx),

            // Model switching
            "switch_model" => self.toggle_model_switcher(cx),

            // Templates & Commands
            "show_templates" => self.toggle_templates_panel(cx),
            "show_commands" => self.toggle_commands_panel(cx),
            "show_context" => self.toggle_context_panel(cx),

            // Session & History
            "session_history" => self.toggle_session_history(cx),
            "resume_last" => self.resume_last_session(cx),
            "toggle_suggestions" => self.toggle_suggestions(cx),

            // Permissions
            "permissions_panel" => self.toggle_permissions_panel(cx),

            // Panels
            "mcp_panel" => self.toggle_mcp_panel(cx),
            "tasks_panel" => self.toggle_tasks_panel(cx),
            "git_panel" => self.toggle_git_panel(cx),

            // Files
            "file_picker" => self.toggle_file_picker(cx),

            // Bookmarks
            "toggle_bookmark" => {
                if let Some(idx) = self.selected_message_index {
                    self.toggle_bookmark(idx, cx);
                }
            }
            "show_bookmarks" => self.toggle_bookmarked_only(cx),
            "next_bookmark" => self.jump_to_next_bookmark(cx),
            "prev_bookmark" => self.jump_to_prev_bookmark(cx),

            // Input mode
            "toggle_multiline" => self.toggle_multiline_input(cx),
            "increase_input_height" => self.increase_input_height(cx),
            "decrease_input_height" => self.decrease_input_height(cx),

            // Session & Metrics
            "session_details" => self.toggle_session_details(cx),
            "toggle_thinking" => self.toggle_thinking(cx),
            "copy_session_id" => {
                if let Some(ref info) = self.session_info {
                    cx.write_to_clipboard(gpui::ClipboardItem::new_string(info.session_id.clone()));
                    self.show_notification(
                        "Session ID copied to clipboard",
                        NotificationType::Success,
                        cx,
                    );
                }
            }
            "cmd_clear" => self.send_slash_command("/clear", cx),

            // Notes & Organization
            "toggle_notes" => self.toggle_notes_panel(cx),
            "toggle_tags" => self.toggle_tags_editor(cx),
            "toggle_favorites" => self.toggle_favorites_panel(cx),
            "save_as_favorite" => {
                // Prompt user would be ideal, for now save with timestamp
                let label = format!("Saved {}", chrono::Utc::now().format("%m/%d %H:%M"));
                self.save_input_as_favorite(label, cx);
            }
            "toggle_pinned" => self.toggle_pinned_panel(cx),
            "toggle_recent_files" => self.toggle_recent_files_panel(cx),
            "toggle_stats" => self.toggle_stats_panel(cx),

            // Focus & Workflow
            "toggle_focus_mode" => self.toggle_focus_mode(cx),
            "quick_resume" => self.toggle_session_history(cx),

            // Input
            "clear_input_history" => self.clear_input_history(cx),
            "toggle_input_hints" => self.toggle_input_hints(cx),

            // Settings
            "quick_settings" => self.toggle_quick_settings(cx),

            // Help
            "show_shortcuts" => self.toggle_shortcuts_help(cx),

            // Summary & Title
            "auto_title" => self.auto_generate_title(cx),
            "ai_title" => self.request_ai_title(cx),
            "request_summary" => self.request_summary(cx),
            "quick_summary" => {
                let summary = self.get_quick_summary();
                self.show_notification(summary, NotificationType::Info, cx);
            }
            "export_summary" => {
                let summary = self.export_shareable_summary();
                cx.write_to_clipboard(ClipboardItem::new_string(summary));
                self.show_notification(
                    "Summary copied to clipboard",
                    NotificationType::Success,
                    cx,
                );
            }

            // Quick File Mentions
            "mention_readme" => self.quick_mention_readme(cx),
            "mention_package" => self.quick_mention_package(cx),
            "mention_cargo" => self.quick_mention_cargo(cx),

            // Session Health & Performance
            "retry_last_request" => self.retry_last_request(cx),
            "check_session_health" => {
                self.calculate_session_health(cx);
                let health = self.stats.health_label();
                self.show_notification(
                    &format!("Session health: {}", health),
                    NotificationType::Info,
                    cx,
                );
            }
            "clear_quick_replies" => {
                self.quick_reply_suggestions.clear();
                self.show_notification("Quick replies cleared", NotificationType::Info, cx);
            }
            "dismiss_tips" => {
                self.panels.onboarding_tips = false;
                self.show_notification("Tips dismissed", NotificationType::Info, cx);
            }

            // Claude Code Skills
            "skill_apex" => self.send_slash_command("/apex", cx),
            "skill_brainstorm" => self.send_slash_command("/brainstorm", cx),
            "skill_explore" => self.send_slash_command("/explore", cx),
            "skill_debug" => self.send_slash_command("/debug", cx),
            "skill_review" => self.send_slash_command("/review", cx),
            "skill_oneshot" => self.send_slash_command("/oneshot", cx),
            "skill_explain" => self.send_slash_command("/explain", cx),
            "skill_refactor" => self.send_slash_command("/refactor", cx),
            "skill_docs" => self.send_slash_command("/docs", cx),
            "skill_ultrathink" => self.send_slash_command("/ultrathink", cx),

            // Git Skills
            "skill_commit" => self.send_slash_command("/commit", cx),
            "skill_create_pr" => self.send_slash_command("/create-pr", cx),
            "skill_fix_pr" => self.send_slash_command("/fix-pr-comments", cx),
            "skill_merge" => self.send_slash_command("/merge", cx),

            _ => {
                tracing::warn!("Unknown palette command: {}", command_id);
            }
        }
        cx.notify();
    }
}
