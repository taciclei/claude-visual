//! Inline Comments for Diff Review
//!
//! Support for adding and managing comments on diff lines.

mod inline_comment;
mod comment_location;
mod comment_thread;
mod diff_comments;
mod tests;

pub use inline_comment::InlineComment;
pub use comment_location::CommentLocation;
pub use comment_thread::CommentThread;
pub use diff_comments::DiffComments;
