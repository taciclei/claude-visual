//! Thread of comments on a specific line

use super::inline_comment::InlineComment;

/// A thread of comments on a specific line
#[derive(Debug, Clone)]
pub struct CommentThread {
    /// Hunk ID this thread is on
    pub hunk_id: usize,
    /// Line index within the hunk
    pub line_index: usize,
    /// Side: 'old' or 'new'
    pub side: String,
    /// Comments in this thread (root + replies)
    pub comments: Vec<InlineComment>,
    /// Is thread collapsed
    pub collapsed: bool,
    /// Is thread resolved
    pub resolved: bool,
}

impl CommentThread {
    /// Create a new thread
    pub fn new(hunk_id: usize, line_index: usize, side: &str) -> Self {
        Self {
            hunk_id,
            line_index,
            side: side.to_string(),
            comments: Vec::new(),
            collapsed: false,
            resolved: false,
        }
    }

    /// Add a comment to this thread
    pub fn add_comment(&mut self, comment: InlineComment) {
        self.comments.push(comment);
    }

    /// Get root comments (not replies)
    pub fn root_comments(&self) -> Vec<&InlineComment> {
        self.comments
            .iter()
            .filter(|c| c.reply_to.is_none())
            .collect()
    }

    /// Get replies to a specific comment
    pub fn replies_to(&self, parent_id: &str) -> Vec<&InlineComment> {
        self.comments
            .iter()
            .filter(|c| c.reply_to.as_deref() == Some(parent_id))
            .collect()
    }

    /// Mark all comments as resolved
    pub fn resolve(&mut self) {
        self.resolved = true;
        for comment in &mut self.comments {
            comment.resolved = true;
        }
    }

    /// Unresolve thread
    pub fn unresolve(&mut self) {
        self.resolved = false;
    }

    /// Get comment count
    pub fn comment_count(&self) -> usize {
        self.comments.len()
    }

    /// Check if thread has unresolved comments
    pub fn has_unresolved(&self) -> bool {
        self.comments.iter().any(|c| !c.resolved)
    }

    /// Remove a comment by ID
    pub fn remove_comment(&mut self, comment_id: &str) -> Option<InlineComment> {
        if let Some(pos) = self.comments.iter().position(|c| c.id == comment_id) {
            Some(self.comments.remove(pos))
        } else {
            None
        }
    }
}
