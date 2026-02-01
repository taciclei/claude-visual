use std::time::{Duration, Instant};
use super::AnnouncementPriority;

/// An announcement for screen readers
#[derive(Debug, Clone)]
pub struct Announcement {
    /// The message to announce
    pub message: String,
    /// Priority level
    pub priority: AnnouncementPriority,
    /// When the announcement was created
    pub created_at: Instant,
    /// How long to keep the announcement visible
    pub duration: Duration,
}

impl Announcement {
    /// Create a new polite announcement
    pub fn polite(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            priority: AnnouncementPriority::Polite,
            created_at: Instant::now(),
            duration: Duration::from_secs(5),
        }
    }

    /// Create a new assertive announcement
    pub fn assertive(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            priority: AnnouncementPriority::Assertive,
            created_at: Instant::now(),
            duration: Duration::from_secs(5),
        }
    }

    /// Set custom duration
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Check if the announcement has expired
    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.duration
    }
}
