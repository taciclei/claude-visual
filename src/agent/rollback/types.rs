//! Types for the rollback system

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Types of operations that can be rolled back
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RollbackOperation {
    /// File was created
    FileCreated { path: PathBuf },
    /// File was modified
    FileModified {
        path: PathBuf,
        original_content: Vec<u8>,
        original_permissions: Option<u32>,
    },
    /// File was deleted
    FileDeleted {
        path: PathBuf,
        original_content: Vec<u8>,
        original_permissions: Option<u32>,
    },
    /// File was renamed/moved
    FileRenamed { from: PathBuf, to: PathBuf },
    /// Directory was created
    DirectoryCreated { path: PathBuf },
    /// Directory was deleted
    DirectoryDeleted {
        path: PathBuf,
        /// Recursive contents
        contents: Vec<RollbackOperation>,
    },
    /// Command was executed
    CommandExecuted {
        command: String,
        args: Vec<String>,
        cwd: PathBuf,
        /// Optional rollback command
        rollback_command: Option<String>,
    },
    /// Git commit was made
    GitCommit {
        repo_path: PathBuf,
        commit_hash: String,
    },
    /// Git branch was created
    GitBranchCreated {
        repo_path: PathBuf,
        branch_name: String,
    },
    /// Database record was inserted
    DatabaseInsert { table: String, record_id: String },
    /// Database record was updated
    DatabaseUpdate {
        table: String,
        record_id: String,
        original_data: serde_json::Value,
    },
    /// Database record was deleted
    DatabaseDelete {
        table: String,
        record_id: String,
        original_data: serde_json::Value,
    },
    /// Custom operation with handler
    Custom {
        name: String,
        data: serde_json::Value,
    },
}

impl RollbackOperation {
    /// Get operation type name
    pub fn operation_type(&self) -> &'static str {
        match self {
            Self::FileCreated { .. } => "file_created",
            Self::FileModified { .. } => "file_modified",
            Self::FileDeleted { .. } => "file_deleted",
            Self::FileRenamed { .. } => "file_renamed",
            Self::DirectoryCreated { .. } => "directory_created",
            Self::DirectoryDeleted { .. } => "directory_deleted",
            Self::CommandExecuted { .. } => "command_executed",
            Self::GitCommit { .. } => "git_commit",
            Self::GitBranchCreated { .. } => "git_branch_created",
            Self::DatabaseInsert { .. } => "database_insert",
            Self::DatabaseUpdate { .. } => "database_update",
            Self::DatabaseDelete { .. } => "database_delete",
            Self::Custom { .. } => "custom",
        }
    }

    /// Get a human-readable description
    pub fn description(&self) -> String {
        match self {
            Self::FileCreated { path } => format!("Created file: {}", path.display()),
            Self::FileModified { path, .. } => format!("Modified file: {}", path.display()),
            Self::FileDeleted { path, .. } => format!("Deleted file: {}", path.display()),
            Self::FileRenamed { from, to } => {
                format!("Renamed: {} -> {}", from.display(), to.display())
            }
            Self::DirectoryCreated { path } => format!("Created directory: {}", path.display()),
            Self::DirectoryDeleted { path, .. } => format!("Deleted directory: {}", path.display()),
            Self::CommandExecuted { command, args, .. } => {
                format!("Executed: {} {}", command, args.join(" "))
            }
            Self::GitCommit { commit_hash, .. } => format!("Git commit: {}", &commit_hash[..8]),
            Self::GitBranchCreated { branch_name, .. } => {
                format!("Created branch: {}", branch_name)
            }
            Self::DatabaseInsert { table, record_id } => {
                format!("Inserted into {}: {}", table, record_id)
            }
            Self::DatabaseUpdate {
                table, record_id, ..
            } => {
                format!("Updated {}: {}", table, record_id)
            }
            Self::DatabaseDelete {
                table, record_id, ..
            } => {
                format!("Deleted from {}: {}", table, record_id)
            }
            Self::Custom { name, .. } => format!("Custom: {}", name),
        }
    }

    /// Check if this operation is reversible
    pub fn is_reversible(&self) -> bool {
        match self {
            Self::CommandExecuted {
                rollback_command, ..
            } => rollback_command.is_some(),
            Self::Custom { .. } => false, // Custom ops need explicit handler
            _ => true,
        }
    }
}

/// A checkpoint in the rollback history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackCheckpoint {
    /// Unique checkpoint ID
    pub id: String,
    /// Name for this checkpoint
    pub name: String,
    /// When the checkpoint was created
    pub created_at: DateTime<Utc>,
    /// Step number in the plan (if applicable)
    pub step_number: Option<usize>,
    /// Operations to rollback (in reverse order)
    pub(crate) operations: Vec<RollbackOperation>,
}

impl RollbackCheckpoint {
    /// Create a new checkpoint
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            created_at: Utc::now(),
            step_number: None,
            operations: Vec::new(),
        }
    }

    /// Create a checkpoint for a plan step
    pub fn for_step(step_number: usize, name: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            created_at: Utc::now(),
            step_number: Some(step_number),
            operations: Vec::new(),
        }
    }

    /// Add an operation to this checkpoint
    pub fn add_operation(&mut self, op: RollbackOperation) {
        self.operations.push(op);
    }

    /// Get number of operations
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Check if checkpoint has any operations
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Check if all operations are reversible
    pub fn is_fully_reversible(&self) -> bool {
        self.operations.iter().all(|op| op.is_reversible())
    }

    /// Get operations reference
    pub(crate) fn operations(&self) -> &[RollbackOperation] {
        &self.operations
    }
}

/// Result of a rollback operation
#[derive(Debug, Clone)]
pub struct RollbackResult {
    /// Whether rollback was successful
    pub success: bool,
    /// Number of operations rolled back
    pub operations_rolled_back: usize,
    /// Operations that failed to rollback
    pub failed_operations: Vec<(RollbackOperation, String)>,
    /// Duration of rollback
    pub duration_ms: u64,
}

impl RollbackResult {
    /// Check if rollback was partial (some operations failed)
    pub fn is_partial(&self) -> bool {
        !self.failed_operations.is_empty() && self.operations_rolled_back > 0
    }
}
