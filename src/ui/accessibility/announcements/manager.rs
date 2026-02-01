use std::collections::VecDeque;
use super::{Announcement, AnnouncementPriority, LiveRegion};

/// Announcement manager for screen reader notifications
#[derive(Debug, Default)]
pub struct AnnouncementManager {
    /// Queue of announcements
    queue: VecDeque<Announcement>,
    /// Maximum queue size
    max_queue_size: usize,
    /// Current announcement being displayed
    current: Option<Announcement>,
}

impl AnnouncementManager {
    /// Create a new announcement manager
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            max_queue_size: 10,
            current: None,
        }
    }

    /// Create with custom max queue size
    pub fn with_max_queue_size(mut self, size: usize) -> Self {
        self.max_queue_size = size;
        self
    }

    /// Add an announcement to the queue
    pub fn announce(&mut self, announcement: Announcement) {
        // If assertive, clear any pending polite announcements
        if announcement.priority == AnnouncementPriority::Assertive {
            self.queue.retain(|a| a.priority == AnnouncementPriority::Assertive);
            self.current = Some(announcement);
            return;
        }

        // Add to queue
        if self.queue.len() < self.max_queue_size {
            self.queue.push_back(announcement);
        }
    }

    /// Announce a polite message
    pub fn polite(&mut self, message: impl Into<String>) {
        self.announce(Announcement::polite(message));
    }

    /// Announce an assertive message
    pub fn assertive(&mut self, message: impl Into<String>) {
        self.announce(Announcement::assertive(message));
    }

    /// Get the current announcement to display
    pub fn current(&self) -> Option<&Announcement> {
        self.current.as_ref()
    }

    /// Get the current message text
    pub fn current_message(&self) -> Option<&str> {
        self.current.as_ref().map(|a| a.message.as_str())
    }

    /// Get the live region type for current announcement
    pub fn current_live_region(&self) -> LiveRegion {
        self.current
            .as_ref()
            .map(|a| LiveRegion::from(a.priority))
            .unwrap_or(LiveRegion::Off)
    }

    /// Update the manager (call periodically to advance queue)
    pub fn tick(&mut self) {
        // Check if current announcement expired
        if let Some(ref current) = self.current {
            if current.is_expired() {
                self.current = None;
            }
        }

        // If no current announcement, get next from queue
        if self.current.is_none() {
            self.current = self.queue.pop_front();
        }
    }

    /// Clear all announcements
    pub fn clear(&mut self) {
        self.queue.clear();
        self.current = None;
    }

    /// Get number of pending announcements
    pub fn pending_count(&self) -> usize {
        self.queue.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_announcement_queue() {
        let mut manager = AnnouncementManager::new();

        manager.polite("First message");
        manager.polite("Second message");

        assert_eq!(manager.pending_count(), 2);
        assert!(manager.current().is_none());

        manager.tick();
        assert_eq!(manager.current_message(), Some("First message"));
        assert_eq!(manager.pending_count(), 1);
    }

    #[test]
    fn test_assertive_priority() {
        let mut manager = AnnouncementManager::new();

        manager.polite("Polite message");
        manager.assertive("Urgent message");

        // Assertive should be current immediately
        assert_eq!(manager.current_message(), Some("Urgent message"));
        // Polite should be cleared
        assert_eq!(manager.pending_count(), 0);
    }

    #[test]
    fn test_live_region() {
        let mut manager = AnnouncementManager::new();

        assert_eq!(manager.current_live_region(), LiveRegion::Off);

        manager.polite("Test");
        manager.tick();
        assert_eq!(manager.current_live_region(), LiveRegion::Polite);

        manager.assertive("Urgent");
        assert_eq!(manager.current_live_region(), LiveRegion::Assertive);
    }
}
