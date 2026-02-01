//! Branch operations

use anyhow::Result;
use git2::BranchType;

use super::Repository;

impl Repository {
    /// List all local branches
    pub fn list_branches(&self) -> Result<Vec<String>> {
        let branches = self.repo.branches(Some(BranchType::Local))?;
        let mut names = Vec::new();

        for branch in branches {
            let (branch, _) = branch?;
            if let Some(name) = branch.name()? {
                names.push(name.to_string());
            }
        }

        Ok(names)
    }

    /// Get the default branch name
    pub fn default_branch(&self) -> String {
        // Try to find main or master
        if self.repo.find_branch("main", BranchType::Local).is_ok() {
            "main".to_string()
        } else if self.repo.find_branch("master", BranchType::Local).is_ok() {
            "master".to_string()
        } else {
            "main".to_string()
        }
    }
}
