//! Storage Cleanup Utilities
//!
//! Provides cleanup functionality for conversations, messages, and other stored data
//! to manage disk space and improve performance.

mod job;
mod scheduler;
mod types;
mod utils;

// Re-export public types
pub use job::CleanupJob;
pub use scheduler::CleanupScheduler;
pub use types::{CleanupConfig, CleanupItem, CleanupStats, CleanupTarget};
pub use utils::{get_available_space_mb, needs_disk_space_cleanup};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_cleanup_config_defaults() {
        let config = CleanupConfig::default();
        assert_eq!(config.max_conversation_age_days, 90);
        assert!(config.auto_cleanup_enabled);
    }

    #[test]
    fn test_cleanup_item_should_cleanup() {
        let config = CleanupConfig::default();

        // Recent item should not be cleaned
        let recent = CleanupItem::conversation("1", "test", 1000, Utc::now() - Duration::days(1));
        assert!(!recent.should_cleanup(&config));

        // Old item should be cleaned
        let old = CleanupItem::conversation("2", "test", 1000, Utc::now() - Duration::days(100));
        assert!(old.should_cleanup(&config));

        // Protected item should not be cleaned
        let protected = old.clone().protect();
        assert!(!protected.should_cleanup(&config));
    }

    #[test]
    fn test_cleanup_stats_format() {
        let mut stats = CleanupStats::new();
        stats.space_freed_bytes = 1_500_000;
        assert_eq!(stats.format_space_freed(), "1.5 MB");

        stats.space_freed_bytes = 500;
        assert_eq!(stats.format_space_freed(), "500 B");
    }

    #[test]
    fn test_cleanup_job_preview() {
        let config = CleanupConfig::default();
        let mut job = CleanupJob::new(config.clone());

        job.add_item(CleanupItem::conversation(
            "1",
            "recent",
            1000,
            Utc::now() - Duration::days(1),
        ));
        job.add_item(CleanupItem::conversation(
            "2",
            "old",
            2000,
            Utc::now() - Duration::days(100),
        ));

        let preview = job.preview();
        assert_eq!(preview.len(), 1);
        assert_eq!(preview[0].id, "2");
    }

    #[test]
    fn test_cleanup_scheduler() {
        let config = CleanupConfig::default();
        let mut scheduler = CleanupScheduler::new(config);

        assert!(!scheduler.is_cleanup_due());
        assert!(scheduler.last_cleanup().is_none());

        scheduler.complete_cleanup();
        assert!(scheduler.last_cleanup().is_some());
    }
}
