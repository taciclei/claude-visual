//! Pool type definitions

use std::sync::atomic::{AtomicUsize, Ordering};

/// Statistics for a pool
#[derive(Debug, Clone, Default)]
pub struct PoolStats {
    /// Total items created
    pub created: usize,
    /// Items currently in pool (available)
    pub available: usize,
    /// Items currently in use
    pub in_use: usize,
    /// Times an item was reused from pool
    pub reused: usize,
    /// Times pool was empty and new item was created
    pub misses: usize,
}

pub(crate) struct PoolStatsInner {
    pub(crate) created: AtomicUsize,
    pub(crate) reused: AtomicUsize,
    pub(crate) misses: AtomicUsize,
}

impl Default for PoolStatsInner {
    fn default() -> Self {
        Self {
            created: AtomicUsize::new(0),
            reused: AtomicUsize::new(0),
            misses: AtomicUsize::new(0),
        }
    }
}

impl PoolStatsInner {
    pub(crate) fn to_stats(&self, available: usize) -> PoolStats {
        let created = self.created.load(Ordering::Relaxed);
        let reused = self.reused.load(Ordering::Relaxed);
        let misses = self.misses.load(Ordering::Relaxed);

        PoolStats {
            created,
            available,
            in_use: created.saturating_sub(available),
            reused,
            misses,
        }
    }
}

/// Message view pool configuration
pub struct MessageViewPoolConfig {
    /// Maximum number of pooled views
    pub max_size: usize,
    /// Pre-warm count
    pub prewarm_count: usize,
}

impl Default for MessageViewPoolConfig {
    fn default() -> Self {
        Self {
            max_size: 50,
            prewarm_count: 10,
        }
    }
}

/// Generic render data for pooled message views
#[derive(Debug, Clone, Default)]
pub struct PooledMessageData {
    /// Message ID
    pub id: String,
    /// Message content
    pub content: String,
    /// Role (user/assistant/system)
    pub role: String,
    /// Whether collapsed
    pub collapsed: bool,
    /// Whether this is a streaming message
    pub is_streaming: bool,
    /// Timestamp
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

impl PooledMessageData {
    /// Create user message data
    pub fn user(id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
            role: "user".to_string(),
            collapsed: false,
            is_streaming: false,
            timestamp: Some(chrono::Utc::now()),
        }
    }

    /// Create assistant message data
    pub fn assistant(id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
            role: "assistant".to_string(),
            collapsed: false,
            is_streaming: false,
            timestamp: Some(chrono::Utc::now()),
        }
    }

    /// Set streaming state
    pub fn streaming(mut self, is_streaming: bool) -> Self {
        self.is_streaming = is_streaming;
        self
    }

    /// Reset for reuse
    pub fn reset(&mut self) {
        self.id.clear();
        self.content.clear();
        self.role.clear();
        self.collapsed = false;
        self.is_streaming = false;
        self.timestamp = None;
    }
}
