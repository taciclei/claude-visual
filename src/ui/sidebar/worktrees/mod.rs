//! Worktree manager panel

mod core;
mod render;
mod types;

pub use types::{WorktreePanel, WorktreePanelEvent};

use gpui::EventEmitter;

impl EventEmitter<WorktreePanelEvent> for WorktreePanel {}
