//! Type definitions for worktree panel

use std::path::PathBuf;
use std::sync::Arc;

use crate::app::state::AppState;
use crate::git::status::FileStatus;
use crate::git::worktree::WorktreeInfo;

/// Events emitted by WorktreePanel
pub enum WorktreePanelEvent {
    /// A worktree was selected
    WorktreeSelected(PathBuf),
    /// User requested to create a new worktree
    CreateWorktreeRequested,
    /// User clicked on a changed file
    FileClicked(String),
    /// User requested to delete a worktree
    DeleteWorktreeRequested(PathBuf),
    /// Send a Claude Code skill command
    SendSkillCommand(String),
}

/// Worktree manager sidebar panel
pub struct WorktreePanel {
    pub(crate) app_state: Arc<AppState>,
    pub(crate) worktrees: Vec<WorktreeInfo>,
    pub(crate) selected_worktree: Option<usize>,
    pub(crate) is_git_repo: bool,
    pub(crate) file_statuses: Vec<FileStatus>,
    pub(crate) current_branch: Option<String>,
    /// Context menu open for worktree at this index
    pub(crate) context_menu_worktree: Option<usize>,
}
