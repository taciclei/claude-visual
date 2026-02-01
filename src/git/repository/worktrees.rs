//! Worktree operations

use std::path::{Path, PathBuf};

use anyhow::Result;
use git2::BranchType;

use super::Repository;
use crate::git::worktree::WorktreeInfo;

impl Repository {
    /// List all worktrees
    pub fn list_worktrees(&self) -> Result<Vec<WorktreeInfo>> {
        let worktrees = self.repo.worktrees()?;
        let mut result = Vec::new();

        // Add main worktree
        if let Some(workdir) = self.repo.workdir() {
            let branch = self.current_branch();
            result.push(WorktreeInfo {
                name: "main".to_string(),
                path: workdir.to_path_buf(),
                branch,
                is_main: true,
                is_locked: false,
            });
        }

        // Add linked worktrees
        for name in worktrees.iter() {
            if let Some(name) = name {
                if let Ok(wt) = self.repo.find_worktree(name) {
                    let wt_path = wt.path().to_path_buf();
                    let branch = self.get_worktree_branch(&wt_path);
                    let is_locked = wt.is_locked().is_ok();

                    result.push(WorktreeInfo {
                        name: name.to_string(),
                        path: wt_path,
                        branch,
                        is_main: false,
                        is_locked,
                    });
                }
            }
        }

        Ok(result)
    }

    /// Get the branch for a worktree path
    fn get_worktree_branch(&self, path: &Path) -> Option<String> {
        let head_path = path.join(".git").join("HEAD");
        if head_path.exists() {
            std::fs::read_to_string(&head_path)
                .ok()
                .and_then(|content| {
                    if content.starts_with("ref: refs/heads/") {
                        Some(content.trim_start_matches("ref: refs/heads/").trim().to_string())
                    } else {
                        None
                    }
                })
        } else {
            None
        }
    }

    /// Create a new worktree
    pub fn create_worktree(&self, name: &str, branch: &str, path: &Path) -> Result<WorktreeInfo> {
        // Find or create the branch
        let branch_ref = if let Ok(branch) = self.repo.find_branch(branch, BranchType::Local) {
            branch.into_reference()
        } else {
            // Create branch from HEAD
            let head = self.repo.head()?;
            let commit = head.peel_to_commit()?;
            self.repo.branch(branch, &commit, false)?.into_reference()
        };

        // Create the worktree
        let wt = self.repo.worktree(
            name,
            path,
            Some(git2::WorktreeAddOptions::new().reference(Some(&branch_ref))),
        )?;

        Ok(WorktreeInfo {
            name: name.to_string(),
            path: wt.path().to_path_buf(),
            branch: Some(branch.to_string()),
            is_main: false,
            is_locked: false,
        })
    }

    /// Remove a worktree
    pub fn remove_worktree(&self, name: &str, force: bool) -> Result<()> {
        let wt = self.repo.find_worktree(name)?;

        if force {
            // Remove directory and prune
            let path = wt.path().to_path_buf();
            if path.exists() {
                std::fs::remove_dir_all(&path)?;
            }
            wt.prune(Some(
                git2::WorktreePruneOptions::new()
                    .valid(true)
                    .working_tree(true),
            ))?;
        } else {
            wt.prune(None)?;
        }

        Ok(())
    }
}
