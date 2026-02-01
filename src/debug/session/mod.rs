//! Debug Session Management
//!
//! High-level debug session handling with state management.

mod types;
mod core;
mod lifecycle;
mod breakpoints;
mod execution;
mod inspection;
mod events;

// Re-export public types
pub use types::{DebugState, SessionEvent, UserBreakpoint};
pub use core::DebugSession;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use crate::debug::client::DapClientConfig;

    #[test]
    fn test_debug_state() {
        assert_eq!(DebugState::Running.icon(), "▶");
        assert_eq!(DebugState::Stopped.icon(), "⏹");
    }

    #[test]
    fn test_user_breakpoint() {
        let bp = UserBreakpoint::new(PathBuf::from("main.rs"), 42)
            .with_condition("x > 10");

        assert_eq!(bp.line, 42);
        assert_eq!(bp.condition, Some("x > 10".to_string()));
        assert!(bp.enabled);
        assert!(!bp.verified);
    }

    #[test]
    fn test_session_creation() {
        let config = DapClientConfig::default();
        let (session, _rx) = DebugSession::new(config);

        assert_eq!(session.state(), DebugState::Idle);
        assert!(session.threads().is_empty());
    }
}
