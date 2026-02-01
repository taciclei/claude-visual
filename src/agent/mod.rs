//! Agent Module
//!
//! Autonomous agent mode for multi-step task execution with planning,
//! tool use, and human oversight.

pub mod executor;
pub mod planner;
pub mod rollback;
pub mod task;

pub use executor::{AgentExecutor, ExecutorEvent, ExecutorState};
pub use planner::{AgentPlanner, Plan, PlanStep};
pub use rollback::{RollbackCheckpoint, RollbackManager, RollbackOperation, RollbackResult};
pub use task::{AgentTask, TaskNode, TaskStatus, TaskTree};
