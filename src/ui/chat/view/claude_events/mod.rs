//! Claude CLI event handling
//!
//! This module contains methods for handling events from the Claude CLI stream,
//! including message streaming, tool execution, and error handling.

mod handler;
mod streaming;
mod formatting;

pub use handler::*;
pub use streaming::*;
pub use formatting::*;
