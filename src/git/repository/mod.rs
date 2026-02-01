//! Git repository operations

mod branches;
mod core;
mod diff;
mod status;
mod worktrees;

pub use core::Repository;
pub use status::RepositoryStatusSummary;
