//! Enhanced Diff Viewer Module
//!
//! Advanced diff visualization with side-by-side view, inline comments,
//! hunk management, and three-way merge support.

mod comments;
mod hunk;
mod side_by_side;

pub use comments::{CommentThread, DiffComments, InlineComment};
pub use hunk::{DiffHunkManager, HunkAction, HunkStatus};
pub use side_by_side::{DiffDisplayMode, SideBySideDiffEvent, SideBySideDiffView};
