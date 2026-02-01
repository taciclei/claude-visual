//! DAP Client Implementation
//!
//! Client for communicating with Debug Adapters via stdin/stdout.

mod types;
mod core;
mod messaging;
mod initialization;
mod execution;
mod inspection;
mod breakpoints;
mod lifecycle;

// Re-export public types
pub use types::{DapClientError, DapClientConfig};
pub use core::DapClient;
