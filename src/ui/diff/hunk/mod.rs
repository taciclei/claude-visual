//! Diff Hunk Management
//!
//! Structures for managing diff hunks with apply/reject functionality.

mod types;
mod line;
mod managed;
mod manager;
mod tests;

// Re-export public types
pub use types::{HunkAction, HunkStatus};
pub use line::HunkLine;
pub use managed::ManagedHunk;
pub use manager::DiffHunkManager;
