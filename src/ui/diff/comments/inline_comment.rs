//! Individual inline comment on a diff line

/// A single comment on a diff line
#[derive(Debug, Clone)]
pub struct InlineComment {
    /// Unique comment ID
    pub id: String,
    /// Author username
    pub author: String,
    /// Comment content (markdown)
    pub content: String,
    /// Timestamp (Unix epoch)
    pub timestamp: u64,
    /// Is this comment resolved
    pub resolved: bool,
    /// Reply to another comment (parent ID)
    pub reply_to: Option<String>,
    /// Has been edited
    pub edited: bool,
}

impl InlineComment {
    /// Create a new comment
    pub fn new(author: String, content: String) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            author,
            content,
            timestamp,
            resolved: false,
            reply_to: None,
            edited: false,
        }
    }

    /// Create a reply to another comment
    pub fn reply(parent_id: &str, author: String, content: String) -> Self {
        let mut comment = Self::new(author, content);
        comment.reply_to = Some(parent_id.to_string());
        comment
    }

    /// Edit comment content
    pub fn edit(&mut self, new_content: String) {
        self.content = new_content;
        self.edited = true;
    }

    /// Resolve this comment
    pub fn resolve(&mut self) {
        self.resolved = true;
    }

    /// Unresolve this comment
    pub fn unresolve(&mut self) {
        self.resolved = false;
    }

    /// Format timestamp as relative time
    pub fn relative_time(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let diff = now.saturating_sub(self.timestamp);

        if diff < 60 {
            "just now".to_string()
        } else if diff < 3600 {
            format!("{}m ago", diff / 60)
        } else if diff < 86400 {
            format!("{}h ago", diff / 3600)
        } else if diff < 604800 {
            format!("{}d ago", diff / 86400)
        } else {
            format!("{}w ago", diff / 604800)
        }
    }
}
