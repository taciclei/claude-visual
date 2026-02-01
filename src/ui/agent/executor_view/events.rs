//! Executor view events

/// Events emitted by the executor view
#[derive(Debug, Clone)]
pub enum ExecutorViewEvent {
    /// Start/resume execution requested
    Start,
    /// Pause execution requested
    Pause,
    /// Cancel execution requested
    Cancel,
    /// Approve current step
    Approve,
    /// Reject current step
    Reject,
}
