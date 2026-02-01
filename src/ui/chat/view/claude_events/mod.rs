//! Claude CLI event handling
//!
//! This module contains methods for handling events from the Claude CLI stream,
//! including message streaming, tool execution, and error handling.

mod formatting;
mod handler;
mod streaming;

pub use formatting::*;
pub use handler::*;
pub use streaming::*;
