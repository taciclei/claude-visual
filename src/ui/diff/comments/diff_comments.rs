//! Manager for all comments in a diff

use std::collections::HashMap;
use super::{comment_location::CommentLocation, comment_thread::CommentThread, inline_comment::InlineComment};

/// Manager for all comments in a diff
#[derive(Debug, Clone)]
pub struct DiffComments {
    /// All comment threads indexed by location
    threads: HashMap<CommentLocation, CommentThread>,
    /// Current user for creating comments
    pub current_user: String,
}

impl Default for DiffComments {
    fn default() -> Self {
        Self::new("User".to_string())
    }
}

impl DiffComments {
    /// Create a new comments manager
    pub fn new(current_user: String) -> Self {
        Self {
            threads: HashMap::new(),
            current_user,
        }
    }

    /// Get or create a thread at a location
    pub fn get_or_create_thread(&mut self, hunk_id: usize, line_index: usize, side: &str) -> &mut CommentThread {
        let location = CommentLocation::new(hunk_id, line_index, side);

        self.threads.entry(location).or_insert_with(|| {
            CommentThread::new(hunk_id, line_index, side)
        })
    }

    /// Get thread at location
    pub fn get_thread(&self, hunk_id: usize, line_index: usize, side: &str) -> Option<&CommentThread> {
        let location = CommentLocation::new(hunk_id, line_index, side);
        self.threads.get(&location)
    }

    /// Get mutable thread at location
    pub fn get_thread_mut(&mut self, hunk_id: usize, line_index: usize, side: &str) -> Option<&mut CommentThread> {
        let location = CommentLocation::new(hunk_id, line_index, side);
        self.threads.get_mut(&location)
    }

    /// Add a new comment
    pub fn add_comment(&mut self, hunk_id: usize, line_index: usize, side: &str, content: String) -> String {
        let comment = InlineComment::new(self.current_user.clone(), content);
        let id = comment.id.clone();

        let thread = self.get_or_create_thread(hunk_id, line_index, side);
        thread.add_comment(comment);

        id
    }

    /// Reply to a comment
    pub fn reply(&mut self, hunk_id: usize, line_index: usize, side: &str, parent_id: &str, content: String) -> Option<String> {
        let comment = InlineComment::reply(parent_id, self.current_user.clone(), content);
        let id = comment.id.clone();

        let thread = self.get_thread_mut(hunk_id, line_index, side)?;
        thread.add_comment(comment);

        Some(id)
    }

    /// Check if location has comments
    pub fn has_comments(&self, hunk_id: usize, line_index: usize, side: &str) -> bool {
        let location = CommentLocation::new(hunk_id, line_index, side);
        self.threads.get(&location).map(|t| !t.comments.is_empty()).unwrap_or(false)
    }

    /// Get comment count at location
    pub fn comment_count(&self, hunk_id: usize, line_index: usize, side: &str) -> usize {
        let location = CommentLocation::new(hunk_id, line_index, side);
        self.threads.get(&location).map(|t| t.comment_count()).unwrap_or(0)
    }

    /// Get all threads for a hunk
    pub fn threads_for_hunk(&self, hunk_id: usize) -> Vec<&CommentThread> {
        self.threads
            .iter()
            .filter(|(loc, _)| loc.hunk_id == hunk_id)
            .map(|(_, thread)| thread)
            .collect()
    }

    /// Get total comment count
    pub fn total_comments(&self) -> usize {
        self.threads.values().map(|t| t.comment_count()).sum()
    }

    /// Get unresolved comment count
    pub fn unresolved_count(&self) -> usize {
        self.threads
            .values()
            .flat_map(|t| &t.comments)
            .filter(|c| !c.resolved)
            .count()
    }

    /// Get all threads
    pub fn all_threads(&self) -> Vec<&CommentThread> {
        self.threads.values().collect()
    }

    /// Resolve all comments in a thread
    pub fn resolve_thread(&mut self, hunk_id: usize, line_index: usize, side: &str) {
        if let Some(thread) = self.get_thread_mut(hunk_id, line_index, side) {
            thread.resolve();
        }
    }

    /// Remove empty threads
    pub fn cleanup(&mut self) {
        self.threads.retain(|_, thread| !thread.comments.is_empty());
    }

    /// Export comments as JSON
    pub fn export_json(&self) -> String {
        // Simple JSON export
        let mut json = String::from("[\n");
        let mut first = true;

        for (location, thread) in &self.threads {
            for comment in &thread.comments {
                if !first {
                    json.push_str(",\n");
                }
                first = false;

                json.push_str(&format!(
                    r#"  {{"hunk_id": {}, "line_index": {}, "side": "{}", "author": "{}", "content": "{}", "timestamp": {}, "resolved": {}}}"#,
                    location.hunk_id,
                    location.line_index,
                    location.side,
                    comment.author,
                    comment.content.replace('"', "\\\"").replace('\n', "\\n"),
                    comment.timestamp,
                    comment.resolved
                ));
            }
        }

        json.push_str("\n]");
        json
    }
}
