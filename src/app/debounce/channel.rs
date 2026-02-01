use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use tokio::sync::mpsc;

/// Debounced channel for async operations
///
/// Useful for debouncing events that trigger async work.
pub struct DebouncedChannel<T> {
    sender: mpsc::Sender<T>,
    interval_ms: u64,
    last_send: AtomicU64,
}

impl<T: Send + 'static> DebouncedChannel<T> {
    /// Create a new debounced channel
    pub fn new(interval: Duration) -> (Self, mpsc::Receiver<T>) {
        let (sender, receiver) = mpsc::channel(16);
        (
            Self {
                sender,
                interval_ms: interval.as_millis() as u64,
                last_send: AtomicU64::new(0),
            },
            receiver,
        )
    }

    /// Try to send a value (may be dropped if too frequent)
    pub fn try_send(&self, value: T) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let last = self.last_send.load(Ordering::SeqCst);
        if now.saturating_sub(last) < self.interval_ms {
            return false;
        }

        if self
            .last_send
            .compare_exchange(last, now, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            self.sender.try_send(value).is_ok()
        } else {
            false
        }
    }

    /// Force send (always sends)
    pub async fn force_send(&self, value: T) -> bool {
        self.last_send.store(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            Ordering::SeqCst,
        );
        self.sender.send(value).await.is_ok()
    }
}
