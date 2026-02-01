//! Helper methods for Workspace

use super::workspace::Workspace;
use crate::project::manager::Project;
use crate::ui::components::toast::Toast;
use gpui::*;

impl Workspace {
    /// Toggle focus mode (hides sidebar and tab bar)
    pub fn toggle_focus_mode(&mut self, cx: &mut Context<Self>) {
        self.focus_mode = !self.focus_mode;
        if self.focus_mode {
            // Hide sidebar when entering focus mode
            self.show_sidebar = false;
        }
        tracing::info!(
            "Focus mode: {}",
            if self.focus_mode {
                "enabled"
            } else {
                "disabled"
            }
        );
        cx.notify();
    }

    /// Check if focus mode is enabled
    pub fn is_focus_mode(&self) -> bool {
        self.focus_mode
    }

    /// Preview file content
    pub(crate) fn preview_file(&mut self, path: std::path::PathBuf, cx: &mut Context<Self>) {
        // Read the file content and display it
        if path.is_file() {
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    tracing::info!("Previewing file: {:?} ({} bytes)", path, content.len());
                    // TODO: Show in preview panel when available
                    // For now, we can add it to the toast notifications
                }
                Err(e) => {
                    tracing::error!("Failed to read file {:?}: {}", path, e);
                }
            }
        }
        cx.notify();
    }

    /// Open a file in the system default editor
    pub(crate) fn open_file_external(&self, path: std::path::PathBuf) {
        #[cfg(target_os = "macos")]
        {
            let _ = std::process::Command::new("open").arg(&path).spawn();
        }
        #[cfg(target_os = "linux")]
        {
            let _ = std::process::Command::new("xdg-open").arg(&path).spawn();
        }
        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("cmd")
                .args(["/c", "start", "", path.to_str().unwrap_or("")])
                .spawn();
        }
        tracing::info!("Opening file externally: {:?}", path);
    }

    /// Add a file to the chat context
    pub(crate) fn add_file_to_chat_context(
        &mut self,
        path: std::path::PathBuf,
        cx: &mut Context<Self>,
    ) {
        if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            let path_str = path.to_string_lossy().to_string();
            chat_view.update(cx, |chat, cx| {
                // Insert @file:path mention into the input
                chat.insert_mention(&format!("@file:{}", path_str), cx);
            });
        }
    }

    /// Show a toast notification
    pub fn show_toast(&mut self, toast: Toast, cx: &mut Context<Self>) {
        self.toast_container.update(cx, |container, cx| {
            container.show(toast, cx);
        });
    }

    /// Update git status for current project
    pub(crate) fn update_git_status(&mut self, path: &std::path::PathBuf, cx: &mut Context<Self>) {
        use crate::git::Repository;
        use crate::ui::chat::view::GitInfo;

        // Try to open git repository
        if let Ok(repo) = Repository::open(path) {
            if let Ok(summary) = repo.status_summary() {
                let git_info = GitInfo {
                    branch: summary.branch,
                    is_dirty: summary.is_dirty,
                    staged_count: summary.staged_count,
                    unstaged_count: summary.unstaged_count,
                    untracked_count: summary.untracked_count,
                    ahead: summary.ahead,
                    behind: summary.behind,
                    last_commit: summary.last_commit,
                    remote: summary.remote,
                };

                // Update chat views with git info
                for chat_view in &self.chat_views {
                    chat_view.update(cx, |view, cx| {
                        view.update_git_info(git_info.clone(), cx);
                    });
                }

                tracing::info!(
                    "Git status updated: branch={}, dirty={}",
                    git_info.branch,
                    git_info.is_dirty
                );
            }
        } else {
            // Clear git info if not a git repository
            for chat_view in &self.chat_views {
                chat_view.update(cx, |view, cx| {
                    view.clear_git_info(cx);
                });
            }
        }
    }

    /// Update the status bar with current workspace state
    pub(in crate::ui::workspace) fn update_status_bar(&mut self, cx: &mut Context<Self>) {
        // Get project info
        let (project_name, project_path) = {
            let manager = self.app_state.project_manager.read(cx);
            if let Some(dir) = self.app_state.current_directory() {
                // Find matching project
                let project = manager.list_projects().ok().and_then(|projects| {
                    let dir_path = std::path::PathBuf::from(dir.clone());
                    projects.into_iter().find(|p| p.path == dir_path)
                });
                if let Some(p) = project {
                    (Some(p.name.clone()), Some(dir.clone()))
                } else {
                    // Use directory name as project name
                    let name = std::path::Path::new(&dir)
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string());
                    (name, Some(dir.clone()))
                }
            } else {
                (None, None)
            }
        };

        // Get chat view state
        let (
            message_count,
            filter_name,
            vim_mode,
            word_wrap,
            line_numbers,
            session_health,
            response_latency,
        ) = if let Some(chat_view) = self.chat_views.get(self.active_chat_index) {
            chat_view.update(cx, |view, cx| {
                let filter = view.message_filter();
                let filter_name = match filter {
                    crate::ui::chat::view::MessageFilter::All => "All",
                    crate::ui::chat::view::MessageFilter::UserOnly => "User",
                    crate::ui::chat::view::MessageFilter::AssistantOnly => "Assistant",
                    crate::ui::chat::view::MessageFilter::ToolsOnly => "Tools",
                }
                .to_string();
                let msg_len = view.messages_len();
                let vim_enabled = view.is_vim_mode_enabled(cx);
                let word_wrap = view.is_word_wrap_enabled();
                let line_nums = view.is_line_numbers_enabled();
                let health = view.get_session_health();
                let latency = view.get_response_latency_ms();
                (
                    msg_len,
                    filter_name,
                    vim_enabled,
                    word_wrap,
                    line_nums,
                    health,
                    latency,
                )
            })
        } else {
            (0, "All".to_string(), false, false, true, 1.0, None)
        };

        // Check if streaming
        let is_streaming = self.cancel_sender.is_some();

        // Update status bar
        self.status_bar.update(cx, |bar, cx| {
            bar.set_project(
                project_name,
                project_path.map(|p| p.to_string_lossy().to_string()),
                cx,
            );
            bar.set_message_count(message_count, cx);
            bar.set_streaming(is_streaming, cx);
            bar.set_vim_mode(vim_mode, cx);
            bar.set_filter(filter_name, cx);
            bar.set_word_wrap(word_wrap, cx);
            bar.set_line_numbers(line_numbers, cx);
            bar.set_session_health(session_health, cx);
            bar.set_response_latency(response_latency, cx);
        });
    }

    /// Handle dropped folders from drag and drop
    pub(crate) fn handle_dropped_folders(
        &mut self,
        paths: Vec<std::path::PathBuf>,
        cx: &mut Context<Self>,
    ) {
        tracing::info!("Dropped {} folder(s)", paths.len());

        for path in paths {
            // Create project from path
            let project = Project::from_path(path.clone());

            // Add to database via project manager
            self.app_state.project_manager.update(cx, |manager, _cx| {
                if let Err(e) = manager.add_project(project) {
                    // Might already exist, which is fine
                    tracing::debug!("Failed to add project (may already exist): {}", e);
                } else {
                    tracing::info!("Added project from drop: {:?}", path);
                }
            });
        }

        // Refresh the projects sidebar
        self.projects_sidebar.update(cx, |sidebar, cx| {
            sidebar.refresh(cx);
        });
    }
}
