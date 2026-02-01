//! Tests for AgentExecutor

#[cfg(test)]
mod tests {
    use crate::agent::executor::core::AgentExecutor;
    use crate::agent::executor::types::{ExecutorState, ExecutorStats};

    #[test]
    fn test_executor_state() {
        let executor = AgentExecutor::new();
        assert_eq!(executor.state(), ExecutorState::Idle);
    }

    #[test]
    fn test_executor_state_transitions() {
        assert!(ExecutorState::Running.can_pause());
        assert!(!ExecutorState::Idle.can_pause());
        assert!(ExecutorState::Paused.can_resume());
        assert!(ExecutorState::WaitingApproval.can_resume());
        assert!(ExecutorState::Completed.is_finished());
        assert!(ExecutorState::Failed.is_finished());
        assert!(ExecutorState::Cancelled.is_finished());
    }

    #[test]
    fn test_executor_stats() {
        let stats = ExecutorStats {
            state: ExecutorState::Running,
            completed_steps: 5,
            total_steps: 10,
            duration_ms: Some(1000),
        };
        assert_eq!(stats.completion_percentage(), 50.0);
    }
}
