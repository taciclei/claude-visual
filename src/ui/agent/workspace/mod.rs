//! Agent Workspace Integration
//!
//! Combines agent UI components (task panel, executor view, plan view) into a
//! cohesive agent mode interface that integrates with the main workspace.

mod types;
mod state;
mod lifecycle;
mod tasks;
mod approval;
mod execution;
mod internal;
mod layout;
mod getters;
mod render;
mod helpers;
mod tests;

pub use types::{
    AgentMode,
    AgentWorkspaceEvent,
    NotificationType,
    AgentLayout,
    AgentSettings,
};
pub use state::AgentWorkspace;
