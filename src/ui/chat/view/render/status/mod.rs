//! Status rendering module for ChatView
//!
//! This module contains status-related render functions organized into:
//! - **latency.rs** - Latency indicator rendering
//! - **stats_bar.rs** - Statistics bar with message counts, tokens, and context progress
//! - **status_header/** - Status header directory module with:
//!   - connection.rs - Connection status data and rendering
//!   - left_section.rs - Left side badges (model, tools, skills, agents, etc.)
//!   - right_section.rs - Right side content (active tasks, indicators)
//!   - helpers.rs - Utility functions (spinners, formatters)
//! - **types.rs** - Status-specific types (currently minimal)

pub mod latency;
pub mod stats_bar;
pub mod status_header;
pub mod types;

// Re-export all render functions
pub use latency::*;
pub use stats_bar::*;
