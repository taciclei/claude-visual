//! Enhanced Diff Viewer Module
//!
//! Advanced diff visualization with side-by-side view, inline comments,
//! hunk management, and three-way merge support.

mod side_by_side;
mod hunk;
mod comments;

pub use side_by_side::{SideBySideDiffView, SideBySideDiffEvent, DiffDisplayMode};
pub use hunk::{DiffHunkManager, HunkAction, HunkStatus};
pub use comments::{InlineComment, CommentThread, DiffComments};
