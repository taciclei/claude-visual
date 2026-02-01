//! Agent Rollback System
//!
//! Provides rollback capability for agent operations, allowing
//! changes made during plan execution to be reversed.

mod checkpoint;
mod execution;
mod manager;
mod query;
mod recording;
mod types;

#[cfg(test)]
mod tests;

pub use manager::RollbackManager;
pub use types::{RollbackCheckpoint, RollbackOperation, RollbackResult};
