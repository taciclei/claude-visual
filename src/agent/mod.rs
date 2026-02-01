//! Agent Module
//!
//! Autonomous agent mode for multi-step task execution with planning,
//! tool use, and human oversight.

pub mod task;
pub mod executor;
pub mod planner;
pub mod rollback;

pub use task::{AgentTask, TaskStatus, TaskTree, TaskNode};
pub use executor::{AgentExecutor, ExecutorState, ExecutorEvent};
pub use planner::{AgentPlanner, Plan, PlanStep};
pub use rollback::{RollbackManager, RollbackOperation, RollbackCheckpoint, RollbackResult};
