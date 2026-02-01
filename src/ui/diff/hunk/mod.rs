//! Diff Hunk Management
//!
//! Structures for managing diff hunks with apply/reject functionality.

mod line;
mod managed;
mod manager;
mod tests;
mod types;

// Re-export public types
pub use line::HunkLine;
pub use managed::ManagedHunk;
pub use manager::DiffHunkManager;
pub use types::{HunkAction, HunkStatus};
