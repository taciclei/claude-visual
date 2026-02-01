//! Core repository operations

use std::path::Path;

use anyhow::Result;
use git2::{Repository as Git2Repository, RepositoryState};

/// Wrapper around git2 repository
pub struct Repository {
    pub(super) repo: Git2Repository,
}

impl Repository {
    /// Open a repository at the given path
    pub fn open(path: &Path) -> Result<Self> {
        let repo = Git2Repository::discover(path)?;
        Ok(Self { repo })
    }

    /// Check if a path is a git repository
    pub fn is_repo(path: &Path) -> bool {
        Git2Repository::discover(path).is_ok()
    }

    /// Get the repository root path
    pub fn path(&self) -> Option<&Path> {
        self.repo.workdir()
    }

    /// Get the current branch name
    pub fn current_branch(&self) -> Option<String> {
        self.repo
            .head()
            .ok()
            .and_then(|head| head.shorthand().map(|s| s.to_string()))
    }

    /// Get repository state
    pub fn state(&self) -> RepositoryState {
        self.repo.state()
    }
}
