//! Debounce utilities
//!
//! Provides debouncing functionality for rate-limiting UI updates.

mod batch;
mod channel;
mod coalescer;
mod debouncer;
mod throttled;

#[cfg(test)]
mod tests;

pub use batch::BatchAccumulator;
pub use channel::DebouncedChannel;
pub use coalescer::UpdateCoalescer;
pub use debouncer::Debouncer;
pub use throttled::ThrottledCallback;
