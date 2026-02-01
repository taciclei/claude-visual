use super::AnnouncementPriority;

/// ARIA live region configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveRegion {
    /// Off - no announcements
    Off,
    /// Polite - wait for user to stop interacting
    Polite,
    /// Assertive - interrupt immediately
    Assertive,
}

impl Default for LiveRegion {
    fn default() -> Self {
        Self::Polite
    }
}

impl From<AnnouncementPriority> for LiveRegion {
    fn from(priority: AnnouncementPriority) -> Self {
        match priority {
            AnnouncementPriority::Polite => LiveRegion::Polite,
            AnnouncementPriority::Assertive => LiveRegion::Assertive,
        }
    }
}
