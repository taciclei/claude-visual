//! Agent Rollback System
//!
//! Provides rollback capability for agent operations, allowing
//! changes made during plan execution to be reversed.

mod types;
mod manager;
mod checkpoint;
mod recording;
mod query;
mod execution;

#[cfg(test)]
mod tests;

pub use types::{RollbackOperation, RollbackCheckpoint, RollbackResult};
pub use manager::RollbackManager;
