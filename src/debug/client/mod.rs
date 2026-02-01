//! DAP Client Implementation
//!
//! Client for communicating with Debug Adapters via stdin/stdout.

mod breakpoints;
mod core;
mod execution;
mod initialization;
mod inspection;
mod lifecycle;
mod messaging;
mod types;

// Re-export public types
pub use core::DapClient;
pub use types::{DapClientConfig, DapClientError};
