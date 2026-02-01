use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use super::Debouncer;

/// UI update coalescer
///
/// Coalesces multiple UI updates into a single render pass.
#[derive(Clone)]
pub struct UpdateCoalescer {
    inner: Arc<UpdateCoalescerInner>,
}

struct UpdateCoalescerInner {
    dirty: AtomicBool,
    update_count: AtomicU64,
    debouncer: Debouncer,
}

impl UpdateCoalescer {
    /// Create a new update coalescer
    pub fn new(min_interval: Duration) -> Self {
        Self {
            inner: Arc::new(UpdateCoalescerInner {
                dirty: AtomicBool::new(false),
                update_count: AtomicU64::new(0),
                debouncer: Debouncer::new(min_interval),
            }),
        }
    }

    /// Create a coalescer for streaming updates (60fps)
    pub fn for_streaming() -> Self {
        Self::new(Duration::from_millis(16))
    }

    /// Mark as needing update
    pub fn mark_dirty(&self) {
        self.inner.dirty.store(true, Ordering::SeqCst);
        self.inner.update_count.fetch_add(1, Ordering::SeqCst);
    }

    /// Check if should render and clear dirty flag
    pub fn should_render(&self) -> bool {
        if !self.inner.dirty.load(Ordering::SeqCst) {
            return false;
        }

        if self.inner.debouncer.should_execute() {
            self.inner.dirty.store(false, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    /// Force render
    pub fn force_render(&self) -> bool {
        if !self.inner.dirty.load(Ordering::SeqCst) {
            return false;
        }
        self.inner.debouncer.force_execute();
        self.inner.dirty.store(false, Ordering::SeqCst);
        true
    }

    /// Check if dirty
    pub fn is_dirty(&self) -> bool {
        self.inner.dirty.load(Ordering::SeqCst)
    }

    /// Get total update count
    pub fn update_count(&self) -> u64 {
        self.inner.update_count.load(Ordering::SeqCst)
    }
}
