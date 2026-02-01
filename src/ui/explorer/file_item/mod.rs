//! File Entry Types
//!
//! Data structures for file tree items.

mod display;
mod entry;
mod operations;
mod tests;
mod types;

pub use entry::FileEntry;
pub use types::{FileType, GitStatus};
