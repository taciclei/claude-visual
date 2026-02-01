//! Event subscription setup for Workspace components

use gpui::*;
use crate::ui::chat::view::{ChatView, ChatViewEvent};
use crate::ui::components::status_bar::StatusBarEvent;
use crate::ui::explorer::FileTreeEvent;
use crate::ui::sidebar::history::HistorySidebarEvent;
use crate::ui::sidebar::projects::ProjectsSidebarEvent;
use crate::ui::sidebar::worktrees::WorktreePanelEvent;
use crate::ui::cloud::TeamPanelEvent;
use crate::ui::tabs::TabBarEvent;
use super::workspace::Workspace;
use super::super::types::SidebarTab;

impl Workspace {
    /// Subscribe to tab bar events
    pub(super) fn subscribe_to_tab_bar(&mut self, cx: &mut Context<Self>) {
        cx.subscribe(&self.tab_bar, |this, _, event: &TabBarEvent, cx| {
            match event {
                TabBarEvent::TabSelected(index) => {
                    this.switch_to_tab(*index, cx);
                }
                TabBarEvent::NewTabRequested => {
                    this.create_new_chat_view(cx);
                }
                TabBarEvent::CloseTabRequested(index) => {
                    this.close_chat_view(*index, cx);
                }
                TabBarEvent::TabsReordered { from, to } => {
                    this.reorder_chat_views(*from, *to, cx);
                }
            }
        })
        .detach();
    }

    /// Subscribe to chat view events
    pub(super) fn subscribe_to_chat_view(&mut self, chat_view: &Entity<ChatView>, cx: &mut Context<Self>) {
        cx.subscribe(chat_view, |this, _, event: &ChatViewEvent, cx| {
            match event {
                ChatViewEvent::Submit(text) => {
                    this.send_message(text.clone(), cx);
                }
                ChatViewEvent::StopRequested => {
                    this.cancel_streaming(cx);
                }
                ChatViewEvent::ExportRequested => {
                    this.export_conversation(cx);
                }
                ChatViewEvent::ThemeToggleRequested => {
                    this.toggle_theme(cx);
                }
                ChatViewEvent::RefreshGitStatus => {
                    if let Some(path) = this.app_state.current_directory() {
                        this.update_git_status(&path, cx);
                    }
                }
                ChatViewEvent::CancelTask(task_id) => {
                    // Handle task cancellation - could signal the Claude CLI to stop a task
                    tracing::info!("Task cancelled: {:?}", task_id);
                }
                ChatViewEvent::OpenFile(path) => {
                    // Open file in system default editor or file explorer
                    tracing::info!("Opening file: {}", path);
                    #[cfg(target_os = "macos")]
                    {
                        let _ = std::process::Command::new("open")
                            .arg(&path)
                            .spawn();
                    }
                    #[cfg(target_os = "linux")]
                    {
                        let _ = std::process::Command::new("xdg-open")
                            .arg(&path)
                            .spawn();
                    }
                    #[cfg(target_os = "windows")]
                    {
                        let _ = std::process::Command::new("cmd")
                            .args(["/C", "start", "", &path])
                            .spawn();
                    }
                }
                ChatViewEvent::FileAttached(path) => {
                    // Track file in context - the chat view handles this internally
                    tracing::debug!("File attached to context: {}", path);
                }
                ChatViewEvent::PermissionResponse { request_id, granted } => {
                    // Send permission response back to Claude CLI
                    // This would be handled by the streaming connection
                    tracing::info!("Permission response: {} = {}", request_id, granted);
                    // TODO: Send response to active Claude CLI process
                    // For now, we log and let the CLI timeout or use auto-approve mode
                }
            }
        })
        .detach();
    }

    /// Subscribe to projects sidebar events
    pub(super) fn subscribe_to_projects_sidebar(&mut self, cx: &mut Context<Self>) {
        cx.subscribe(&self.projects_sidebar, |this, _, event: &ProjectsSidebarEvent, cx| {
            match event {
                ProjectsSidebarEvent::ProjectSelected(id, path) => {
                    this.app_state.set_current_directory(Some(path.clone()));

                    // Check for project theme override
                    if let Ok(Some(theme_variant)) = this.app_state.project_manager.read(cx).get_theme_override(id) {
                        this.app_state.theme.update(cx, |theme, _| {
                            theme.set_variant(theme_variant);
                        });
                        tracing::info!("Applied project theme override: {:?}", theme_variant);
                    }

                    // Load file tree with project root
                    this.file_tree.update(cx, |tree, cx| {
                        tree.set_root(path.clone(), cx);
                    });

                    // Switch to Files tab to show the file tree
                    this.sidebar_tab = SidebarTab::Files;

                    // Refresh worktree panel when project changes
                    this.worktree_panel.update(cx, |panel, cx| {
                        panel.refresh(cx);
                    });

                    // Update git status for the new project
                    this.update_git_status(path, cx);

                    // Update status bar with new project info
                    this.update_status_bar(cx);
                    tracing::info!("Selected project: {:?}", path);
                }
                ProjectsSidebarEvent::AddProjectRequested => {
                    this.open_project_picker(cx);
                }
                ProjectsSidebarEvent::FilesDropped(paths) => {
                    this.handle_dropped_folders(paths.clone(), cx);
                }
                ProjectsSidebarEvent::SendSkillCommand(command) => {
                    tracing::info!("Sending skill command from projects panel: {}", command);
                    this.send_skill_command(command, cx);
                }
            }
        })
        .detach();
    }

    /// Subscribe to file tree events
    pub(super) fn subscribe_to_file_tree(&mut self, cx: &mut Context<Self>) {
        cx.subscribe(&self.file_tree, |this, _, event: &FileTreeEvent, cx| {
            match event {
                FileTreeEvent::FileSelected(path) => {
                    tracing::info!("File selected: {:?}", path);
                    // Show file preview in the preview panel
                    this.preview_file(path.clone(), cx);
                }
                FileTreeEvent::FileOpened(path) => {
                    tracing::info!("File opened: {:?}", path);
                    // Open file in system default editor
                    this.open_file_external(path.clone());
                }
                FileTreeEvent::FileAddedToContext(path) => {
                    tracing::info!("File added to context: {:?}", path);
                    // Add file as mention to the chat input
                    this.add_file_to_chat_context(path.clone(), cx);
                }
                _ => {
                    // Handle other file tree events (rename, delete, etc.)
                }
            }
        })
        .detach();
    }

    /// Subscribe to history sidebar events
    pub(super) fn subscribe_to_history_sidebar(&mut self, cx: &mut Context<Self>) {
        cx.subscribe(&self.history_sidebar, |this, _, event: &HistorySidebarEvent, cx| {
            match event {
                HistorySidebarEvent::ConversationSelected(id) => {
                    if let Some(chat_view) = this.chat_views.get(this.active_chat_index) {
                        chat_view.update(cx, |chat, cx| {
                            chat.load_conversation(id, cx);
                        });
                    }
                    tracing::info!("Loaded conversation: {}", id);
                }
                HistorySidebarEvent::DeleteConversation(id) => {
                    tracing::info!("Deleted conversation: {}", id);
                    // Refresh history after delete
                    this.history_sidebar.update(cx, |history, cx| {
                        history.refresh(cx);
                    });
                }
                HistorySidebarEvent::SendSkillCommand(command) => {
                    tracing::info!("Sending skill command from history panel: {}", command);
                    this.send_skill_command(command, cx);
                }
                HistorySidebarEvent::ResumeSession(session_id) => {
                    tracing::info!("Resuming session: {}", session_id);
                    this.send_skill_command(&format!("/resume {}", session_id), cx);
                }
            }
        })
        .detach();
    }

    /// Subscribe to worktree panel events
    pub(super) fn subscribe_to_worktree_panel(&mut self, cx: &mut Context<Self>) {
        cx.subscribe(&self.worktree_panel, |this, _, event: &WorktreePanelEvent, cx| {
            match event {
                WorktreePanelEvent::WorktreeSelected(path) => {
                    tracing::info!("Switched to worktree: {:?}", path);
                }
                WorktreePanelEvent::CreateWorktreeRequested => {
                    tracing::info!("Create worktree requested - TODO: implement dialog");
                    // TODO: Implement worktree creation dialog
                }
                WorktreePanelEvent::FileClicked(path) => {
                    tracing::info!("File clicked: {}", path);
                    this.show_diff_preview(path.clone(), cx);
                }
                WorktreePanelEvent::DeleteWorktreeRequested(path) => {
                    tracing::info!("Delete worktree requested: {:?}", path);
                    // TODO: Implement worktree deletion with confirmation
                }
                WorktreePanelEvent::SendSkillCommand(command) => {
                    tracing::info!("Sending skill command from git panel: {}", command);
                    this.send_skill_command(command, cx);
                }
            }
        })
        .detach();
    }

    /// Subscribe to team panel events
    pub(super) fn subscribe_to_team_panel(&mut self, cx: &mut Context<Self>) {
        cx.subscribe(&self.team_panel, |this, _, event: &TeamPanelEvent, cx| {
            match event {
                TeamPanelEvent::CreateTeam { name, description } => {
                    tracing::info!("Create team requested: {} - {:?}", name, description);
                    // TODO: Implement team creation via TeamManager
                }
                TeamPanelEvent::SelectTeam(id) => {
                    tracing::info!("Team selected: {}", id);
                }
                TeamPanelEvent::InviteMember { team_id, email, role, .. } => {
                    tracing::info!("Invite member {} to team {} as {:?}", email, team_id, role);
                }
                TeamPanelEvent::OpenProject(id) => {
                    tracing::info!("Open project requested: {}", id);
                }
                TeamPanelEvent::AcceptInvitation(id) => {
                    tracing::info!("Accept invitation: {}", id);
                }
                TeamPanelEvent::DeclineInvitation(id) => {
                    tracing::info!("Decline invitation: {}", id);
                }
                _ => {}
            }
        })
        .detach();
    }

    /// Subscribe to status bar events
    pub(super) fn subscribe_to_status_bar(&mut self, cx: &mut Context<Self>) {
        cx.subscribe(&self.status_bar, |this, _, event: &StatusBarEvent, cx| {
            match event {
                StatusBarEvent::OpenModelSwitcher => {
                    // Toggle model switcher in active chat view
                    if let Some(chat_view) = this.chat_views.get(this.active_chat_index) {
                        chat_view.update(cx, |view, cx| {
                            view.toggle_model_switcher(cx);
                        });
                    }
                }
                StatusBarEvent::StopStreaming => {
                    // Stop streaming in active chat view
                    if let Some(chat_view) = this.chat_views.get(this.active_chat_index) {
                        chat_view.update(cx, |view, cx| {
                            view.stop_streaming(cx);
                        });
                    }
                }
                StatusBarEvent::ToggleVimMode => {
                    // Toggle vim mode via settings
                    this.app_state.settings.update(cx, |settings, _| {
                        settings.editor.vim_mode = !settings.editor.vim_mode;
                    });
                    this.update_status_bar(cx);
                }
                StatusBarEvent::ItemClicked(_) => {}
            }
        })
        .detach();
    }
}
