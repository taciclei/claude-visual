//! Core functionality for worktree panel

use std::path::PathBuf;
use std::sync::Arc;

use gpui::*;

use crate::app::state::AppState;

use super::types::{WorktreePanel, WorktreePanelEvent};

impl WorktreePanel {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            worktrees: Vec::new(),
            selected_worktree: None,
            is_git_repo: false,
            file_statuses: Vec::new(),
            current_branch: None,
            context_menu_worktree: None,
        }
    }

    /// Refresh worktree list and file status
    pub fn refresh(&mut self, cx: &mut Context<Self>) {
        self.worktrees.clear();
        self.file_statuses.clear();
        self.is_git_repo = false;
        self.current_branch = None;

        if let Some(cwd) = self.app_state.current_directory() {
            if let Ok(repo) = crate::git::repository::Repository::open(&cwd) {
                self.is_git_repo = true;
                self.current_branch = repo.current_branch();
                self.worktrees = repo.list_worktrees().unwrap_or_default();
                self.file_statuses = repo.status().unwrap_or_default();
            }
        }
        cx.notify();
    }

    /// Select a worktree
    pub fn select_worktree(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(worktree) = self.worktrees.get(index) {
            self.selected_worktree = Some(index);
            let path = worktree.path.clone();
            self.app_state.set_current_directory(Some(path.clone()));
            cx.emit(WorktreePanelEvent::WorktreeSelected(path));
            cx.notify();
        }
    }

    /// Request to create a new worktree
    pub fn create_worktree(&mut self, cx: &mut Context<Self>) {
        cx.emit(WorktreePanelEvent::CreateWorktreeRequested);
    }

    /// Show context menu for a worktree
    pub(super) fn show_context_menu(&mut self, index: usize, cx: &mut Context<Self>) {
        self.context_menu_worktree = Some(index);
        cx.notify();
    }

    /// Hide context menu
    pub(super) fn hide_context_menu(&mut self, cx: &mut Context<Self>) {
        if self.context_menu_worktree.is_some() {
            self.context_menu_worktree = None;
            cx.notify();
        }
    }

    /// Copy branch name to clipboard
    pub(super) fn copy_branch_name(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(worktree) = self.worktrees.get(index) {
            let branch = worktree.branch.clone().unwrap_or_else(|| "detached".to_string());
            cx.write_to_clipboard(gpui::ClipboardItem::new_string(branch));
            tracing::info!("Copied branch name to clipboard");
        }
        self.context_menu_worktree = None;
        cx.notify();
    }

    /// Request to delete a worktree
    pub(super) fn request_delete_worktree(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(worktree) = self.worktrees.get(index) {
            // Don't allow deleting main worktree
            if worktree.is_main {
                tracing::warn!("Cannot delete main worktree");
            } else {
                cx.emit(WorktreePanelEvent::DeleteWorktreeRequested(worktree.path.clone()));
            }
        }
        self.context_menu_worktree = None;
        cx.notify();
    }

    /// Copy worktree path to clipboard
    pub(super) fn copy_worktree_path(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(worktree) = self.worktrees.get(index) {
            let path = worktree.path.display().to_string();
            cx.write_to_clipboard(gpui::ClipboardItem::new_string(path));
            tracing::info!("Copied worktree path to clipboard");
        }
        self.context_menu_worktree = None;
        cx.notify();
    }
}
