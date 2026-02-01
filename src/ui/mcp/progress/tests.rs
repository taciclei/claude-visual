//! Tests for tool progress tracking

use super::types::{ActiveExecution, ExecutionPhase};

#[test]
fn test_active_execution() {
    let execution = ActiveExecution::new(
        "test-1".to_string(),
        "read_file".to_string(),
        "fs".to_string(),
    );

    assert!(execution.phase.is_active());
    assert_eq!(execution.phase, ExecutionPhase::Preparing);
}

#[test]
fn test_execution_phase_is_active() {
    assert!(ExecutionPhase::Preparing.is_active());
    assert!(ExecutionPhase::Executing.is_active());
    assert!(ExecutionPhase::Processing.is_active());
    assert!(!ExecutionPhase::Completed.is_active());
    assert!(!ExecutionPhase::Failed.is_active());
    assert!(!ExecutionPhase::Cancelled.is_active());
}

#[test]
fn test_execution_phase_display() {
    assert_eq!(ExecutionPhase::Preparing.as_str(), "Preparing...");
    assert_eq!(ExecutionPhase::Executing.as_str(), "Executing...");
    assert_eq!(ExecutionPhase::Processing.as_str(), "Processing...");
    assert_eq!(ExecutionPhase::Completed.as_str(), "Completed");
    assert_eq!(ExecutionPhase::Failed.as_str(), "Failed");
    assert_eq!(ExecutionPhase::Cancelled.as_str(), "Cancelled");
}
