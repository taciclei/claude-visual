//! Panel render functions for ChatView
//!
//! This module contains all the panel/modal overlay render functions,
//! each in their own file for better organization.
//!
//! Panels included:
//! - **suggestions** - Contextual suggestions bar and quick templates bar
//! - **error_bar** - Error retry bar with smart suggestions
//! - **session_history** - Recent sessions panel
//! - **permissions** - Permissions management panel
//! - **mcp** - MCP servers panel
//! - **commands** - Commands/palette panel
//! - **templates** - Prompt templates panel
//! - **context** - Context management panel
//! - **export** - Export conversation panel
//! - **notes** - Session notes panel
//! - **favorites** - Favorite prompts panel
//! - **recent_files** - Recent files panel
//! - **pinned** - Pinned messages panel
//! - **stats** - Statistics panel
//! - **quick_settings** - Quick settings panel
//! - **tags** - Tags editor panel
//! - **tasks** - Active tasks panel
//! - **file_picker** - File picker panel
//! - **session_details** - Session details panel

pub mod suggestions;
pub mod error_bar;
pub mod session_history;
pub mod permissions;
pub mod mcp;
pub mod commands;
pub mod templates;  // Now a directory with submodules
pub mod context;
pub mod export;
pub mod notes;
pub mod favorites;
pub mod recent_files;
pub mod pinned;
pub mod stats;
pub mod quick_settings;
pub mod tags;
pub mod tasks;
pub mod file_picker;
pub mod session_details;
