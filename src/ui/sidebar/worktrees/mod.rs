//! Worktree manager panel

mod types;
mod core;
mod render;

pub use types::{WorktreePanel, WorktreePanelEvent};

use gpui::EventEmitter;

impl EventEmitter<WorktreePanelEvent> for WorktreePanel {}
