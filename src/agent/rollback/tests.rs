//! Tests for the rollback system

use std::path::PathBuf;

use super::types::*;
use super::manager::RollbackManager;

#[test]
fn test_rollback_operation_description() {
    let op = RollbackOperation::FileCreated {
        path: PathBuf::from("/test/file.txt"),
    };
    assert!(op.description().contains("Created file"));
    assert!(op.is_reversible());
}

#[test]
fn test_checkpoint_creation() {
    let mut checkpoint = RollbackCheckpoint::new("test");
    assert!(checkpoint.is_empty());

    checkpoint.add_operation(RollbackOperation::FileCreated {
        path: PathBuf::from("/test/file.txt"),
    });

    assert!(!checkpoint.is_empty());
    assert_eq!(checkpoint.operation_count(), 1);
}

#[test]
fn test_rollback_manager_checkpoint_flow() {
    let mut manager = RollbackManager::new();

    manager.begin_checkpoint("step 1");
    manager.record_file_created("/test/new.txt").unwrap();
    let id = manager.commit_checkpoint().unwrap();

    assert_eq!(manager.checkpoint_count(), 1);
    assert!(manager.get_checkpoint(&id).is_some());
}

#[test]
fn test_rollback_manager_step_checkpoints() {
    let mut manager = RollbackManager::new();

    manager.begin_step_checkpoint(1, "step 1 operation");
    manager.record_file_created("/test/file1.txt").unwrap();
    manager.commit_checkpoint();

    manager.begin_step_checkpoint(1, "step 1 another");
    manager.record_file_created("/test/file2.txt").unwrap();
    manager.commit_checkpoint();

    manager.begin_step_checkpoint(2, "step 2 operation");
    manager.record_file_created("/test/file3.txt").unwrap();
    manager.commit_checkpoint();

    assert_eq!(manager.checkpoints_for_step(1).len(), 2);
    assert_eq!(manager.checkpoints_for_step(2).len(), 1);
}

#[test]
fn test_rollback_manager_discard() {
    let mut manager = RollbackManager::new();

    manager.begin_checkpoint("test");
    manager.record_file_created("/test/file.txt").unwrap();
    manager.discard_checkpoint();

    assert_eq!(manager.checkpoint_count(), 0);
}

#[test]
fn test_operation_types() {
    assert_eq!(
        RollbackOperation::FileCreated { path: PathBuf::new() }.operation_type(),
        "file_created"
    );
    assert_eq!(
        RollbackOperation::GitCommit {
            repo_path: PathBuf::new(),
            commit_hash: String::new()
        }.operation_type(),
        "git_commit"
    );
}
