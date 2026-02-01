//! Git worktree management

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Information about a git worktree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorktreeInfo {
    /// Worktree name
    pub name: String,
    /// Path to worktree
    pub path: PathBuf,
    /// Associated branch
    pub branch: Option<String>,
    /// Whether this is the main worktree
    pub is_main: bool,
    /// Whether the worktree is locked
    pub is_locked: bool,
}

impl WorktreeInfo {
    /// Get display name (branch name or worktree name)
    pub fn display_name(&self) -> &str {
        self.branch.as_deref().unwrap_or(&self.name)
    }

    /// Check if the worktree path exists
    pub fn exists(&self) -> bool {
        self.path.exists()
    }
}

/// Options for creating a worktree
#[derive(Debug, Clone, Default)]
pub struct WorktreeOptions {
    /// Create a new branch
    pub create_branch: bool,
    /// Branch to checkout or create
    pub branch: Option<String>,
    /// Lock the worktree after creation
    pub lock: bool,
    /// Force creation even if branch doesn't exist
    pub force: bool,
}

impl WorktreeOptions {
    /// Create default options
    pub fn new() -> Self {
        Self::default()
    }

    /// Set branch name
    pub fn branch(mut self, branch: impl Into<String>) -> Self {
        self.branch = Some(branch.into());
        self
    }

    /// Set create_branch flag
    pub fn create_branch(mut self, create: bool) -> Self {
        self.create_branch = create;
        self
    }

    /// Set lock flag
    pub fn lock(mut self, lock: bool) -> Self {
        self.lock = lock;
        self
    }

    /// Set force flag
    pub fn force(mut self, force: bool) -> Self {
        self.force = force;
        self
    }
}
