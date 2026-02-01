//! Git-related types

/// Represents an active subagent task
#[derive(Debug, Clone)]
pub struct ActiveTask {
    pub task_id: Option<String>,
    pub description: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// Progress percentage (0-100) if available
    pub progress: Option<u8>,
    /// Current status message
    pub status: Option<String>,
}

/// Git repository information
#[derive(Debug, Clone, Default)]
pub struct GitInfo {
    /// Current branch name
    pub branch: String,
    /// Whether there are uncommitted changes
    pub is_dirty: bool,
    /// Number of staged files
    pub staged_count: usize,
    /// Number of unstaged changes
    pub unstaged_count: usize,
    /// Number of untracked files
    pub untracked_count: usize,
    /// Commits ahead of remote
    pub ahead: usize,
    /// Commits behind remote
    pub behind: usize,
    /// Last commit message (short)
    pub last_commit: Option<String>,
    /// Remote name
    pub remote: Option<String>,
}

impl GitInfo {
    /// Get a status summary string
    pub fn status_summary(&self) -> String {
        let mut parts = vec![];
        if self.staged_count > 0 {
            parts.push(format!("+{}", self.staged_count));
        }
        if self.unstaged_count > 0 {
            parts.push(format!("~{}", self.unstaged_count));
        }
        if self.untracked_count > 0 {
            parts.push(format!("?{}", self.untracked_count));
        }
        if parts.is_empty() {
            "clean".to_string()
        } else {
            parts.join(" ")
        }
    }

    /// Get sync status string
    pub fn sync_status(&self) -> Option<String> {
        match (self.ahead, self.behind) {
            (0, 0) => None,
            (a, 0) => Some(format!("↑{}", a)),
            (0, b) => Some(format!("↓{}", b)),
            (a, b) => Some(format!("↑{} ↓{}", a, b)),
        }
    }
}
