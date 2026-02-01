//! Tab management methods for Workspace

use super::workspace::Workspace;
use crate::ui::chat::view::{ChatView, ChatViewEvent};
use gpui::*;

impl Workspace {
    /// Get active chat view
    pub(crate) fn active_chat_view(&self) -> Option<&Entity<ChatView>> {
        self.chat_views.get(self.active_chat_index)
    }

    /// Create a new chat view and add it to the list
    pub(crate) fn create_new_chat_view(&mut self, cx: &mut Context<Self>) {
        let chat_view = cx.new(|cx| ChatView::new(self.app_state.clone(), cx));

        // Subscribe to its events
        cx.subscribe(&chat_view, |this, _, event: &ChatViewEvent, cx| {
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
                        let _ = std::process::Command::new("open").arg(&path).spawn();
                    }
                    #[cfg(target_os = "linux")]
                    {
                        let _ = std::process::Command::new("xdg-open").arg(&path).spawn();
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
                ChatViewEvent::PermissionResponse {
                    request_id,
                    granted,
                } => {
                    // Send permission response back to Claude CLI
                    tracing::info!("Permission response: {} = {}", request_id, granted);
                }
            }
        })
        .detach();

        self.chat_views.push(chat_view);
        self.active_chat_index = self.chat_views.len() - 1;
        self.update_status_bar(cx);
        cx.notify();
    }

    /// Switch to a specific tab
    pub(crate) fn switch_to_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.chat_views.len() {
            self.active_chat_index = index;
            self.update_status_bar(cx);
            cx.notify();
        }
    }

    /// Close a chat view
    pub(crate) fn close_chat_view(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.chat_views.len() <= 1 {
            // Don't close the last view, just clear it
            if let Some(chat_view) = self.chat_views.first() {
                chat_view.update(cx, |chat, cx| {
                    chat.clear(cx);
                });
            }
            return;
        }

        if index < self.chat_views.len() {
            self.chat_views.remove(index);

            // Adjust active index
            if self.active_chat_index >= self.chat_views.len() {
                self.active_chat_index = self.chat_views.len() - 1;
            } else if self.active_chat_index > index {
                self.active_chat_index -= 1;
            }

            cx.notify();
        }
    }

    /// Reorder chat views
    pub(crate) fn reorder_chat_views(&mut self, from: usize, to: usize, cx: &mut Context<Self>) {
        if from < self.chat_views.len() && to <= self.chat_views.len() {
            let chat_view = self.chat_views.remove(from);
            let insert_at = to.min(self.chat_views.len());
            self.chat_views.insert(insert_at, chat_view);

            // Update active index to follow the moved tab if it was active
            if self.active_chat_index == from {
                self.active_chat_index = insert_at;
            } else if from < self.active_chat_index && to >= self.active_chat_index {
                self.active_chat_index -= 1;
            } else if from > self.active_chat_index && to <= self.active_chat_index {
                self.active_chat_index += 1;
            }

            cx.notify();
        }
    }
}
