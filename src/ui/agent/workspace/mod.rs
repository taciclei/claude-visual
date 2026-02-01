//! Agent Workspace Integration
//!
//! Combines agent UI components (task panel, executor view, plan view) into a
//! cohesive agent mode interface that integrates with the main workspace.

mod approval;
mod execution;
mod getters;
mod helpers;
mod internal;
mod layout;
mod lifecycle;
mod render;
mod state;
mod tasks;
mod tests;
mod types;

pub use state::AgentWorkspace;
pub use types::{AgentLayout, AgentMode, AgentSettings, AgentWorkspaceEvent, NotificationType};
