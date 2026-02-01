//! Git integration module

pub mod repository;
pub mod status;
pub mod worktree;

pub use repository::{Repository, RepositoryStatusSummary};
