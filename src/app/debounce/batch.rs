use std::time::Duration;

use parking_lot::Mutex;

use super::Debouncer;

/// Batch accumulator for grouping rapid updates
///
/// Useful for batching multiple small updates into larger ones.
pub struct BatchAccumulator<T> {
    items: Mutex<Vec<T>>,
    debouncer: Debouncer,
    max_batch_size: usize,
}

impl<T: Clone + Send + 'static> BatchAccumulator<T> {
    /// Create a new batch accumulator
    pub fn new(interval: Duration, max_batch_size: usize) -> Self {
        Self {
            items: Mutex::new(Vec::with_capacity(max_batch_size)),
            debouncer: Debouncer::new(interval),
            max_batch_size,
        }
    }

    /// Add an item to the batch
    ///
    /// Returns Some(batch) if the batch should be flushed.
    pub fn add(&self, item: T) -> Option<Vec<T>> {
        let mut items = self.items.lock();
        items.push(item);

        // Flush if max size reached or debounce interval passed
        if items.len() >= self.max_batch_size || self.debouncer.should_execute() {
            let batch = std::mem::take(&mut *items);
            Some(batch)
        } else {
            None
        }
    }

    /// Force flush the batch
    pub fn flush(&self) -> Vec<T> {
        let mut items = self.items.lock();
        self.debouncer.force_execute();
        std::mem::take(&mut *items)
    }

    /// Get current batch size
    pub fn len(&self) -> usize {
        self.items.lock().len()
    }

    /// Check if batch is empty
    pub fn is_empty(&self) -> bool {
        self.items.lock().is_empty()
    }
}
