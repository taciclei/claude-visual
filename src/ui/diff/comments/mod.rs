//! Inline Comments for Diff Review
//!
//! Support for adding and managing comments on diff lines.

mod comment_location;
mod comment_thread;
mod diff_comments;
mod inline_comment;
mod tests;

pub use comment_location::CommentLocation;
pub use comment_thread::CommentThread;
pub use diff_comments::DiffComments;
pub use inline_comment::InlineComment;
