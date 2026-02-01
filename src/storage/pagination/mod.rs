//! Message Pagination
//!
//! Provides pagination utilities for loading messages in batches.

mod cursor;
mod request;
mod state;
mod types;
mod window;

// Re-export public types
pub use cursor::Cursor;
pub use request::PaginationRequest;
pub use state::PaginationState;
pub use types::{PageInfo, PaginatedResult, PaginationDirection};
pub use window::MessageWindow;
