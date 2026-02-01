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

mod bookmarks;
pub mod claude_events;
pub mod commands;
mod commands_logic;
pub mod context;
mod context_usage;
mod core;
mod draft;
pub mod errors;
pub mod favorites;
pub mod git;
pub mod handlers;
mod input_focus;
pub mod mcp;
mod message_ops;
pub mod messages;
pub mod models;
pub mod notifications;
mod panel_toggles;
pub mod panels;
pub mod permissions;
mod reactions;
pub mod render;
pub mod search;
mod search_logic;
mod selection;
pub mod session;
mod streaming;
pub mod suggestions;
pub mod summary;
pub mod tags;
pub mod templates;
pub mod thinking;
pub mod tips;
mod toggles;
pub mod types;
mod vim_fab;

// Re-export everything from core (the main ChatView)
pub use core::*;
pub use suggestions::{ContextualSuggestion, QuickReplySuggestion};
pub use types::*;
