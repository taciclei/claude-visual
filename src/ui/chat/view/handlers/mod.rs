//! Event handlers for ChatView
//!
//! This module contains keyboard, mouse, scroll, and input event handlers,
//! organized into logical submodules:
//!
//! - **palette** - Command palette and keyboard event handlers
//! - **input** - Input configuration handlers (multiline, height, hints)
//! - **history** - Input history navigation and management
//! - **scroll** - Scroll event handlers and auto-scroll
//! - **drag** - Mouse and drag event handlers

pub mod drag;
pub mod history;
pub mod input;
pub mod palette;
pub mod scroll;
