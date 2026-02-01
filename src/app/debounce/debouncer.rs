use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use parking_lot::Mutex;

/// Debouncer for rate-limiting function calls
///
/// Useful for throttling UI updates during rapid streaming events.
pub struct Debouncer {
    /// Minimum interval between calls
    interval: Duration,
    /// Last execution time
    last_execution: Mutex<Option<Instant>>,
    /// Pending call flag
    pending: AtomicBool,
}

impl Debouncer {
    /// Create a new debouncer with the given interval
    pub fn new(interval: Duration) -> Self {
        Self {
            interval,
            last_execution: Mutex::new(None),
            pending: AtomicBool::new(false),
        }
    }

    /// Create a debouncer for streaming UI updates (default 16ms ≈ 60fps)
    pub fn for_streaming() -> Self {
        Self::new(Duration::from_millis(16))
    }

    /// Create a debouncer for search input (default 150ms)
    pub fn for_search() -> Self {
        Self::new(Duration::from_millis(150))
    }

    /// Create a debouncer for resize events (default 50ms)
    pub fn for_resize() -> Self {
        Self::new(Duration::from_millis(50))
    }

    /// Check if we should execute now
    ///
    /// Returns true if enough time has passed since the last execution.
    pub fn should_execute(&self) -> bool {
        let mut last = self.last_execution.lock();
        let now = Instant::now();

        match *last {
            Some(last_time) if now.duration_since(last_time) < self.interval => {
                // Too soon, mark as pending
                self.pending.store(true, Ordering::SeqCst);
                false
            }
            _ => {
                // Execute now
                *last = Some(now);
                self.pending.store(false, Ordering::SeqCst);
                true
            }
        }
    }

    /// Force execution and reset state
    pub fn force_execute(&self) {
        let mut last = self.last_execution.lock();
        *last = Some(Instant::now());
        self.pending.store(false, Ordering::SeqCst);
    }

    /// Check if there's a pending execution
    pub fn has_pending(&self) -> bool {
        self.pending.load(Ordering::SeqCst)
    }

    /// Clear pending state
    pub fn clear_pending(&self) {
        self.pending.store(false, Ordering::SeqCst);
    }

    /// Get time until next allowed execution
    pub fn time_until_next(&self) -> Option<Duration> {
        let last = self.last_execution.lock();
        match *last {
            Some(last_time) => {
                let elapsed = Instant::now().duration_since(last_time);
                if elapsed < self.interval {
                    Some(self.interval - elapsed)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
