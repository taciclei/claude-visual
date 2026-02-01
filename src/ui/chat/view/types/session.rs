//! Session types

/// Recent session info for quick resume
#[derive(Debug, Clone)]
pub struct RecentSession {
    /// Session ID
    pub session_id: String,
    /// Session title/summary
    pub title: String,
    /// Last activity timestamp
    pub last_active: chrono::DateTime<chrono::Utc>,
    /// Number of messages
    pub message_count: usize,
    /// Model used
    pub model: String,
    /// Working directory
    pub cwd: String,
}
