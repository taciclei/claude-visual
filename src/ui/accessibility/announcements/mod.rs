//! Screen reader announcements
//!
//! Provides a way to announce status changes and important events to screen readers.

mod announcement;
mod common;
mod live_region;
mod manager;
mod priority;

pub use announcement::Announcement;
pub use common::CommonAnnouncements;
pub use live_region::LiveRegion;
pub use manager::AnnouncementManager;
pub use priority::AnnouncementPriority;
