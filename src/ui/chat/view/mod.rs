//! ChatView module - split for better organization
//!
//! This module contains the ChatView component split into logical submodules:
//!
//! ## Module Structure
//!
//! - **core.rs** (~19k lines) - Main ChatView struct, types, and primary implementation:
//!   - ChatView struct with all state fields
//!   - Event types (ChatViewEvent, NotificationType, etc.)
//!   - Helper types (MessageFilter, CommandCategory, ErrorCategory, etc.)
//!   - Message handling and streaming logic
//!   - Keyboard/mouse event handlers
//!   - Context management
//!   - impl Render for ChatView
//!
//! - **panels/** - Individual panel modules (extracted from render_panels.rs):
//!   - **suggestions.rs** - Contextual suggestions bar and quick templates bar
//!   - **error_bar.rs** - Error retry bar with smart suggestions
//!   - **session_history.rs** - Recent sessions panel
//!   - **permissions.rs** - Permissions management panel
//!   - **mcp.rs** - MCP servers panel
//!   - **commands.rs** - Commands/palette panel
//!   - **templates.rs** - Prompt templates panel
//!   - **context.rs** - Context management panel
//!   - **export.rs** - Export conversation panel
//!   - **notes.rs** - Session notes panel
//!   - **favorites.rs** - Favorite prompts panel
//!   - **recent_files.rs** - Recent files panel
//!   - **pinned.rs** - Pinned messages panel
//!   - **stats.rs** - Statistics panel
//!   - **quick_settings.rs** - Quick settings panel
//!   - **tags.rs** - Tags editor panel
//!   - **tasks.rs** - Active tasks panel
//!   - **file_picker.rs** - File picker panel
//!   - **session_details.rs** - Session details panel
//!
//! - **git.rs** (~400 lines) - Git integration and commands (extracted):
//!   - toggle_git_panel, update_git_info, clear_git_info
//!   - refresh_git_status
//!   - send_tool_command, request_code_review, create_pr
//!   - show_pr_comments, show_status
//!   - render_git_panel - Git status panel UI
//!

mod core;
pub mod types;
pub mod render;
pub mod panels;
pub mod messages;
pub mod suggestions;
pub mod handlers;
pub mod claude_events;
pub mod session;
pub mod context;
pub mod errors;
pub mod tags;
pub mod summary;
pub mod commands;
mod commands_logic;
pub mod permissions;
pub mod git;
pub mod notifications;
pub mod search;
mod search_logic;
pub mod thinking;
pub mod mcp;
pub mod favorites;
pub mod templates;
pub mod models;
mod toggles;
pub mod tips;
mod draft;
mod reactions;
mod selection;
mod streaming;
mod context_usage;
mod panel_toggles;
mod bookmarks;
mod vim_fab;
mod message_ops;
mod input_focus;

// Re-export everything from core (the main ChatView)
pub use core::*;
pub use types::*;
pub use suggestions::{ContextualSuggestion, QuickReplySuggestion};
