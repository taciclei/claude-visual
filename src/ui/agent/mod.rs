//! Agent UI Module
//!
//! UI components for agent mode execution, task trees, and plan visualization.

pub mod task_panel;
pub mod executor_view;
pub mod plan_view;
pub mod workspace;

pub use task_panel::{TaskPanel, TaskPanelEvent};
pub use executor_view::{ExecutorView, ExecutorViewEvent};
pub use plan_view::PlanView;
pub use workspace::{
    AgentWorkspace,
    AgentWorkspaceEvent,
    AgentMode,
    AgentLayout,
    AgentSettings,
    NotificationType,
};
