//! Cleanup scheduling

use chrono::{DateTime, Duration, Utc};
use super::types::CleanupConfig;

/// Cleanup scheduler
pub struct CleanupScheduler {
    /// Configuration
    pub(crate) config: CleanupConfig,
    /// Last cleanup time
    pub(crate) last_cleanup: Option<DateTime<Utc>>,
    /// Next scheduled cleanup
    pub(crate) next_cleanup: Option<DateTime<Utc>>,
}

impl CleanupScheduler {
    /// Create a new scheduler
    pub fn new(config: CleanupConfig) -> Self {
        let next = if config.auto_cleanup_enabled {
            Some(Utc::now() + Duration::hours(config.auto_cleanup_interval_hours as i64))
        } else {
            None
        };

        Self {
            config,
            last_cleanup: None,
            next_cleanup: next,
        }
    }

    /// Check if cleanup is due
    pub fn is_cleanup_due(&self) -> bool {
        if !self.config.auto_cleanup_enabled {
            return false;
        }

        match self.next_cleanup {
            Some(next) => Utc::now() >= next,
            None => false,
        }
    }

    /// Mark cleanup as completed
    pub fn complete_cleanup(&mut self) {
        let now = Utc::now();
        self.last_cleanup = Some(now);

        if self.config.auto_cleanup_enabled {
            self.next_cleanup = Some(now + Duration::hours(self.config.auto_cleanup_interval_hours as i64));
        }
    }

    /// Get time until next cleanup
    pub fn time_until_next(&self) -> Option<Duration> {
        self.next_cleanup.map(|next| {
            let now = Utc::now();
            if next > now {
                next.signed_duration_since(now)
            } else {
                Duration::zero()
            }
        })
    }

    /// Get last cleanup time
    pub fn last_cleanup(&self) -> Option<DateTime<Utc>> {
        self.last_cleanup
    }

    /// Update configuration
    pub fn set_config(&mut self, config: CleanupConfig) {
        self.config = config;
        if self.config.auto_cleanup_enabled {
            self.next_cleanup = Some(Utc::now() + Duration::hours(self.config.auto_cleanup_interval_hours as i64));
        } else {
            self.next_cleanup = None;
        }
    }
}
