//! Status operations

use anyhow::Result;
use git2::BranchType;

use super::Repository;
use crate::git::status::FileStatus;

impl Repository {
    /// Get file statuses
    pub fn status(&self) -> Result<Vec<FileStatus>> {
        let statuses = self.repo.statuses(None)?;
        let mut result = Vec::new();

        for entry in statuses.iter() {
            if let Some(path) = entry.path() {
                result.push(FileStatus::from_git_status(path, entry.status()));
            }
        }

        Ok(result)
    }

    /// Get comprehensive repository status summary
    pub fn status_summary(&self) -> Result<RepositoryStatusSummary> {
        let branch = self.current_branch().unwrap_or_else(|| "HEAD".to_string());
        let statuses = self.status()?;

        let mut staged_count = 0;
        let mut unstaged_count = 0;
        let mut untracked_count = 0;

        for status in &statuses {
            if status.is_staged() {
                staged_count += 1;
            }
            if status.is_unstaged() {
                if status.workdir_status == Some(crate::git::status::FileStatusKind::New) {
                    untracked_count += 1;
                } else {
                    unstaged_count += 1;
                }
            }
        }

        // Try to get ahead/behind counts from remote tracking branch
        let (ahead, behind) = self.get_ahead_behind().unwrap_or((0, 0));

        // Get last commit message
        let last_commit = self
            .repo
            .head()
            .ok()
            .and_then(|head| head.peel_to_commit().ok())
            .map(|commit| commit.summary().map(|s| s.to_string()).unwrap_or_default());

        // Get remote name
        let remote = self
            .repo
            .find_remote("origin")
            .ok()
            .and_then(|r| r.url().map(|s| s.to_string()));

        Ok(RepositoryStatusSummary {
            branch,
            is_dirty: staged_count > 0 || unstaged_count > 0 || untracked_count > 0,
            staged_count,
            unstaged_count,
            untracked_count,
            ahead,
            behind,
            last_commit,
            remote,
        })
    }

    /// Get ahead/behind counts relative to upstream
    fn get_ahead_behind(&self) -> Result<(usize, usize)> {
        let head = self.repo.head()?;
        let local_oid = head
            .target()
            .ok_or_else(|| anyhow::anyhow!("No local OID"))?;

        // Try to find upstream branch
        let branch = self.repo.find_branch(
            head.shorthand()
                .ok_or_else(|| anyhow::anyhow!("No branch name"))?,
            BranchType::Local,
        )?;

        let upstream = branch.upstream()?;
        let upstream_oid = upstream
            .get()
            .target()
            .ok_or_else(|| anyhow::anyhow!("No upstream OID"))?;

        let (ahead, behind) = self.repo.graph_ahead_behind(local_oid, upstream_oid)?;
        Ok((ahead, behind))
    }
}

/// Summary of repository status
#[derive(Debug, Clone)]
pub struct RepositoryStatusSummary {
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
    /// Remote URL
    pub remote: Option<String>,
}
