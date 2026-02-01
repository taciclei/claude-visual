//! Message-related methods for ChatView
//!
//! This module contains all methods related to message manipulation, selection,
//! navigation, editing, groups, and statistics.
//!
//! Note: Bookmark, pinning, and filtering methods have been moved to view/bookmarks.rs

mod selection;
mod navigation;
mod editing;
mod groups;
mod statistics;

// Re-export all impl blocks
pub use selection::*;
pub use navigation::*;
pub use editing::*;
pub use groups::*;
pub use statistics::*;
