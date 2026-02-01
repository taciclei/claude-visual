/// Announcement priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnnouncementPriority {
    /// Background information (can be interrupted)
    Polite,
    /// Important information (interrupts other announcements)
    Assertive,
}

impl Default for AnnouncementPriority {
    fn default() -> Self {
        Self::Polite
    }
}
