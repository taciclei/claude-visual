//! Watch Expressions View
//!
//! UI for managing and evaluating watch expressions during debugging.

mod types;
mod events;
mod core;
mod render;

pub use types::{WatchExpression, WatchChild};
pub use events::WatchViewEvent;
pub use core::WatchView;
