//! Screen reader announcements
//!
//! Provides a way to announce status changes and important events to screen readers.

mod priority;
mod announcement;
mod live_region;
mod manager;
mod common;

pub use priority::AnnouncementPriority;
pub use announcement::Announcement;
pub use live_region::LiveRegion;
pub use manager::AnnouncementManager;
pub use common::CommonAnnouncements;
