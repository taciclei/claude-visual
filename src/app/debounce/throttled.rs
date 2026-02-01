use std::time::Duration;

use parking_lot::Mutex;

use super::Debouncer;

/// Throttled callback wrapper
///
/// Wraps a callback to ensure it's not called more frequently than the interval.
pub struct ThrottledCallback<T> {
    debouncer: Debouncer,
    latest_value: Mutex<Option<T>>,
}

impl<T: Clone + Send + 'static> ThrottledCallback<T> {
    /// Create a new throttled callback
    pub fn new(interval: Duration) -> Self {
        Self {
            debouncer: Debouncer::new(interval),
            latest_value: Mutex::new(None),
        }
    }

    /// Update with a new value
    ///
    /// Returns true if the callback should fire immediately.
    pub fn update(&self, value: T) -> bool {
        let should_fire = self.debouncer.should_execute();
        if should_fire {
            // Will fire immediately, no need to store
            false
        } else {
            // Store for later
            *self.latest_value.lock() = Some(value);
            true
        }
    }

    /// Get the latest stored value (for trailing call)
    pub fn take_latest(&self) -> Option<T> {
        self.latest_value.lock().take()
    }

    /// Check if there's a pending value
    pub fn has_pending(&self) -> bool {
        self.latest_value.lock().is_some()
    }
}
