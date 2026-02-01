//! Entity and Resource Pooling
//!
//! Provides pooling for GPUI entities to enable reuse and reduce allocation overhead.

mod core;
mod guard;
mod specialized;
mod types;

#[cfg(test)]
mod tests;

// Re-export public API
pub use core::{shared_pool, shared_pool_with_reset, ObjectPool, SharedPool};
pub use guard::PooledItem;
pub use specialized::{StringPool, VecPool};
pub use types::{MessageViewPoolConfig, PoolStats, PooledMessageData};
