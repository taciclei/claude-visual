//! Watch Expressions View
//!
//! UI for managing and evaluating watch expressions during debugging.

mod core;
mod events;
mod render;
mod types;

pub use core::WatchView;
pub use events::WatchViewEvent;
pub use types::{WatchChild, WatchExpression};
