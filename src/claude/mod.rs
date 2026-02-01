//! Claude Code CLI integration
//!
//! This module handles spawning and communicating with the Claude Code CLI process.

pub mod client;
pub mod message;
pub mod streaming;

pub use client::ClaudeClient;
