//! Core cleanup types and data structures

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Cleanup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupConfig {
    /// Maximum age for inactive conversations (days)
    pub max_conversation_age_days: u32,
    /// Maximum number of conversations to keep
    pub max_conversations: usize,
    /// Maximum messages per conversation to keep (0 = unlimited)
    pub max_messages_per_conversation: usize,
    /// Compress messages older than this many days
    pub compress_after_days: u32,
    /// Delete attachments older than this many days
    pub delete_attachments_after_days: u32,
    /// Minimum free disk space to maintain (MB)
    pub min_free_space_mb: u64,
    /// Enable automatic cleanup
    pub auto_cleanup_enabled: bool,
    /// Auto cleanup interval (hours)
    pub auto_cleanup_interval_hours: u32,
}

impl Default for CleanupConfig {
    fn default() -> Self {
        Self {
            max_conversation_age_days: 90,
            max_conversations: 1000,
            max_messages_per_conversation: 500,
            compress_after_days: 30,
            delete_attachments_after_days: 60,
            min_free_space_mb: 500,
            auto_cleanup_enabled: true,
            auto_cleanup_interval_hours: 24,
        }
    }
}

impl CleanupConfig {
    /// Aggressive cleanup for low disk space
    pub fn aggressive() -> Self {
        Self {
            max_conversation_age_days: 30,
            max_conversations: 100,
            max_messages_per_conversation: 100,
            compress_after_days: 7,
            delete_attachments_after_days: 14,
            min_free_space_mb: 1000,
            auto_cleanup_enabled: true,
            auto_cleanup_interval_hours: 6,
        }
    }

    /// Conservative cleanup
    pub fn conservative() -> Self {
        Self {
            max_conversation_age_days: 365,
            max_conversations: 10000,
            max_messages_per_conversation: 0, // Unlimited
            compress_after_days: 90,
            delete_attachments_after_days: 180,
            min_free_space_mb: 100,
            auto_cleanup_enabled: true,
            auto_cleanup_interval_hours: 168, // Weekly
        }
    }
}

/// Cleanup target type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CleanupTarget {
    /// Old conversations
    Conversations,
    /// Old messages within conversations
    Messages,
    /// Attachments (images, files)
    Attachments,
    /// Cached data
    Cache,
    /// Logs
    Logs,
    /// Temporary files
    Temporary,
}

/// Item to be cleaned up
#[derive(Debug, Clone)]
pub struct CleanupItem {
    /// Item ID
    pub id: String,
    /// Item type
    pub target: CleanupTarget,
    /// Item name for display
    pub name: String,
    /// Size in bytes
    pub size_bytes: u64,
    /// Last accessed time
    pub last_accessed: DateTime<Utc>,
    /// Path (if file-based)
    pub path: Option<PathBuf>,
    /// Whether this item is protected
    pub is_protected: bool,
}

impl CleanupItem {
    /// Create a conversation cleanup item
    pub fn conversation(
        id: impl Into<String>,
        name: impl Into<String>,
        size: u64,
        last_accessed: DateTime<Utc>,
    ) -> Self {
        Self {
            id: id.into(),
            target: CleanupTarget::Conversations,
            name: name.into(),
            size_bytes: size,
            last_accessed,
            path: None,
            is_protected: false,
        }
    }

    /// Create an attachment cleanup item
    pub fn attachment(
        id: impl Into<String>,
        path: PathBuf,
        size: u64,
        last_accessed: DateTime<Utc>,
    ) -> Self {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "attachment".to_string());

        Self {
            id: id.into(),
            target: CleanupTarget::Attachments,
            name,
            size_bytes: size,
            last_accessed,
            path: Some(path),
            is_protected: false,
        }
    }

    /// Mark as protected
    pub fn protect(mut self) -> Self {
        self.is_protected = true;
        self
    }

    /// Check if item should be cleaned up based on config
    pub fn should_cleanup(&self, config: &CleanupConfig) -> bool {
        if self.is_protected {
            return false;
        }

        let now = Utc::now();
        let age = now.signed_duration_since(self.last_accessed);

        match self.target {
            CleanupTarget::Conversations => {
                age > Duration::days(config.max_conversation_age_days as i64)
            }
            CleanupTarget::Messages => {
                age > Duration::days(config.max_conversation_age_days as i64)
            }
            CleanupTarget::Attachments => {
                age > Duration::days(config.delete_attachments_after_days as i64)
            }
            CleanupTarget::Cache | CleanupTarget::Temporary => age > Duration::days(7),
            CleanupTarget::Logs => age > Duration::days(30),
        }
    }
}

/// Cleanup statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CleanupStats {
    /// Items scanned
    pub items_scanned: usize,
    /// Items cleaned up
    pub items_cleaned: usize,
    /// Space freed (bytes)
    pub space_freed_bytes: u64,
    /// Errors encountered
    pub errors: usize,
    /// Items skipped (protected)
    pub items_skipped: usize,
    /// Duration of cleanup (ms)
    pub duration_ms: u64,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl CleanupStats {
    /// Create new stats
    pub fn new() -> Self {
        Self {
            timestamp: Utc::now(),
            ..Default::default()
        }
    }

    /// Format space freed for display
    pub fn format_space_freed(&self) -> String {
        let bytes = self.space_freed_bytes;
        if bytes >= 1_000_000_000 {
            format!("{:.1} GB", bytes as f64 / 1_000_000_000.0)
        } else if bytes >= 1_000_000 {
            format!("{:.1} MB", bytes as f64 / 1_000_000.0)
        } else if bytes >= 1_000 {
            format!("{:.1} KB", bytes as f64 / 1_000.0)
        } else {
            format!("{} B", bytes)
        }
    }

    /// Get success rate
    pub fn success_rate(&self) -> f32 {
        if self.items_scanned == 0 {
            return 100.0;
        }
        let success = self.items_scanned - self.errors;
        (success as f32 / self.items_scanned as f32) * 100.0
    }
}
