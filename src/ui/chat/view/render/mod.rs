//! Render module - Split render functions for better organization
//!
//! This module contains all the render_* methods for ChatView organized by functionality:
//!
//! - **status.rs** - Status header, stats bar, and latency indicator
//! - **toolbar.rs** - Messages toolbar, quick actions bar, and quick actions
//! - **navigation.rs** - Navigation bar, scroll to bottom, and FAB
//! - **context_menu.rs** - Context menu rendering
//! - **welcome.rs** - Welcome tips, starter prompts, and recent sessions cards
//! - **indicators.rs** - File drop zone, session health, onboarding tips, and draft indicator
//! - **hints.rs** - Contextual hints, quick actions, and quick reply suggestions
//! - **search.rs** - Search bar rendering
//! - **notifications.rs** - Notifications rendering
//! - **shortcuts.rs** - Shortcuts help overlay
//! - **palette.rs** - Command palette rendering
//! - **git.rs** - Git quick actions
//! - **context_bar.rs** - Context files bar and context progress bar
//! - **streaming_suggestions.rs** - Contextual skill suggestions during/after streaming
//! - **dialogs.rs** - Confirmation dialogs (clear conversation, etc.)

pub mod context_bar;
pub mod context_menu;
pub mod dialogs;
pub mod git;
pub mod hints;
pub mod indicators;
pub mod navigation;
pub mod notifications;
pub mod palette;
pub mod search;
pub mod shortcuts;
pub mod status;
pub mod streaming_suggestions;
pub mod toolbar;
pub mod welcome;

// Re-export all render functions (they're already in impl blocks, so just ensure modules are loaded)
pub use context_bar::*;
pub use context_menu::*;
pub use dialogs::*;
pub use git::*;
pub use hints::*;
pub use indicators::*;
pub use navigation::*;
pub use notifications::*;
pub use palette::*;
pub use search::*;
pub use shortcuts::*;
pub use status::*;
pub use streaming_suggestions::*;
pub use toolbar::*;
pub use welcome::*;
