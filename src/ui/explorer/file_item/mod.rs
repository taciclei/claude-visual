//! File Entry Types
//!
//! Data structures for file tree items.

mod types;
mod entry;
mod display;
mod operations;
mod tests;

pub use types::{FileType, GitStatus};
pub use entry::FileEntry;
