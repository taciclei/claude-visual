//! Message Pagination
//!
//! Provides pagination utilities for loading messages in batches.

mod types;
mod cursor;
mod request;
mod window;
mod state;

// Re-export public types
pub use types::{PageInfo, PaginationDirection, PaginatedResult};
pub use cursor::Cursor;
pub use request::PaginationRequest;
pub use window::MessageWindow;
pub use state::PaginationState;
