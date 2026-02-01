//! Command execution for Workspace

use gpui::*;
use crate::project::manager::Project;
use crate::ui::cloud::TeamViewMode;
use super::workspace::Workspace;
use super::super::types::SidebarTab;

impl Workspace {
    /// Execute a command by ID
    pub(in crate::ui::workspace) fn execute_command(&mut self, command_id: &str, cx: &mut Context<Self>) {
        tracing::info!("Executing command: {}", command_id);
        match command_id {
            // Chat commands
            "new_conversation" => self.new_conversation(cx),
            "clear_conversation" => self.new_conversation(cx),
            "export_markdown" => self.export_conversation(cx),
            "search_conversations" => self.switch_sidebar_tab(SidebarTab::History, cx),
            // View commands
            "toggle_sidebar" => self.toggle_sidebar(cx),
            "toggle_theme" => {
                self.app_state.theme.update(cx, |theme, _cx| {
                    theme.toggle_mode();
                });
            }
            "show_projects" => self.switch_sidebar_tab(SidebarTab::Projects, cx),
            "show_history" => self.switch_sidebar_tab(SidebarTab::History, cx),
            "show_git" => self.switch_sidebar_tab(SidebarTab::Git, cx),
            "show_team" => self.switch_sidebar_tab(SidebarTab::Team, cx),
            // Project commands
            "open_project" => self.open_project_picker(cx),
            // Claude Code Skills - Implementation
            "skill_apex" => self.send_skill_command("/apex", cx),
            "skill_ultrathink" => self.send_skill_command("/ultrathink", cx),
            "skill_oneshot" => self.send_skill_command("/oneshot", cx),
            "skill_refactor" => self.send_skill_command("/refactor", cx),
            "skill_clean_code" => self.send_skill_command("/clean-code", cx),
            // Claude Code Skills - Research & Exploration
            "skill_explore" => self.send_skill_command("/explore", cx),
            "skill_search" => self.send_skill_command("/search", cx),
            "skill_explain" => self.send_skill_command("/explain", cx),
            "skill_brainstorm" => self.send_skill_command("/brainstorm", cx),
            "skill_docs" => self.send_skill_command("/docs", cx),
            // Claude Code Skills - Debugging
            "skill_debug" => self.send_skill_command("/debug", cx),
            "skill_ci_fixer" => self.send_skill_command("/ci-fixer", cx),
            // Claude Code Skills - Git Operations
            "skill_commit" => self.send_skill_command("/commit", cx),
            "skill_create_pr" => self.send_skill_command("/create-pr", cx),
            "skill_review" => self.send_skill_command("/review", cx),
            "skill_merge" => self.send_skill_command("/merge", cx),
            "skill_fix_pr" => self.send_skill_command("/fix-pr-comments", cx),
            // Claude CLI commands
            "cli_resume" => self.send_skill_command("/resume", cx),
            "cli_usage" => self.send_skill_command("/usage", cx),
            "cli_memory" => self.send_skill_command("/memory", cx),
            "cli_compact" => self.send_skill_command("/compact", cx),
            "cli_model" => self.send_skill_command("/model", cx),
            "cli_config" => self.send_skill_command("/config", cx),
            "cli_permissions" => self.send_skill_command("/permissions", cx),
            "cli_doctor" => self.send_skill_command("/doctor", cx),
            "cli_think" => self.send_skill_command("/think", cx),
            "cli_status" => self.send_skill_command("/status", cx),
            // Team commands
            "team_create" => {
                self.switch_sidebar_tab(SidebarTab::Team, cx);
                self.team_panel.update(cx, |panel, cx| {
                    panel.open_create_dialog(cx);
                });
            }
            "team_invite" => {
                self.switch_sidebar_tab(SidebarTab::Team, cx);
                self.team_panel.update(cx, |panel, cx| {
                    panel.open_invite_dialog(cx);
                });
            }
            "team_activity" => {
                self.switch_sidebar_tab(SidebarTab::Team, cx);
                self.team_panel.update(cx, |panel, cx| {
                    panel.set_view_mode(TeamViewMode::TeamDetails, cx);
                });
            }
            "team_analytics" => {
                self.switch_sidebar_tab(SidebarTab::Team, cx);
                self.team_panel.update(cx, |panel, cx| {
                    panel.set_view_mode(TeamViewMode::TeamDetails, cx);
                });
            }
            // App commands
            "settings" => self.show_settings_modal(cx),
            "quit" => cx.quit(),
            _ => {
                tracing::warn!("Unknown command: {}", command_id);
            }
        }
    }

    /// Send a skill command to the active chat view
    pub(in crate::ui::workspace) fn send_skill_command(&mut self, command: &str, cx: &mut Context<Self>) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                view.input.update(cx, |input, cx| {
                    input.set_text(command.to_string(), cx);
                });
                // Emit submit event
                cx.emit(crate::ui::chat::view::types::ChatViewEvent::Submit(command.to_string()));
            });
        }
    }

    /// Clear the active chat view and start new conversation
    pub fn new_conversation(&mut self, cx: &mut Context<Self>) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |chat, cx| {
                chat.clear(cx);
            });
            self.update_status_bar(cx);
        }
    }

    /// Export conversation to Markdown file
    pub fn export_conversation(&mut self, cx: &mut Context<Self>) {
        // Get the markdown content from chat view
        let markdown = if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.read(cx).export_to_markdown()
        } else {
            return;
        };

        if markdown.is_empty() {
            tracing::info!("No messages to export");
            return;
        }

        // Generate a default filename
        let default_filename = format!(
            "claude-conversation-{}.md",
            chrono::Local::now().format("%Y%m%d-%H%M%S")
        );

        cx.background_executor().spawn(async move {
            let file = rfd::AsyncFileDialog::new()
                .set_title("Export Conversation")
                .set_file_name(&default_filename)
                .add_filter("Markdown", &["md"])
                .add_filter("All Files", &["*"])
                .save_file()
                .await;

            if let Some(file) = file {
                let path = file.path();
                match std::fs::write(path, &markdown) {
                    Ok(_) => tracing::info!("Conversation exported to {:?}", path),
                    Err(e) => tracing::error!("Failed to export conversation: {}", e),
                }
            }
        }).detach();
    }

    /// Open native folder picker to add a project
    pub fn open_project_picker(&mut self, cx: &mut Context<Self>) {
        cx.spawn(async move |this, cx| {
            // Use rfd for native folder dialog
            let folder = rfd::AsyncFileDialog::new()
                .set_title("Select Project Folder")
                .pick_folder()
                .await;

            if let Some(folder) = folder {
                let path = folder.path().to_path_buf();
                tracing::info!("Selected project folder: {:?}", path);

                // Create project from path
                let project = Project::from_path(path);

                // Add to database and refresh sidebar
                let _ = this.update(cx, |workspace, cx| {
                    // Add project via project manager
                    workspace.app_state.project_manager.update(cx, |manager, _cx| {
                        if let Err(e) = manager.add_project(project) {
                            tracing::error!("Failed to add project: {}", e);
                        } else {
                            tracing::info!("Project added successfully");
                        }
                    });

                    // Refresh the projects sidebar
                    workspace.projects_sidebar.update(cx, |sidebar, cx| {
                        sidebar.refresh(cx);
                    });
                });
            }
        })
        .detach();
    }
}
