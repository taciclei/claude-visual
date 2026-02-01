//! Debug Adapter Protocol Types
//!
//! Protocol definitions for DAP communication.

mod messages;
mod arguments;
mod types;
mod events;
mod capabilities;

// Re-export all public types
pub use messages::{DapMessage, DapRequest, DapResponse, DapEvent};
pub use arguments::{InitializeArguments, LaunchArguments, AttachArguments};
pub use types::{
    Source, BreakpointLocation, Breakpoint, StackFrame, Scope,
    Variable, VariablePresentationHint, Thread,
};
pub use events::{StoppedEventBody, OutputEventBody, TerminatedEventBody};
pub use capabilities::{Capabilities, ExceptionBreakpointsFilter};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dap_request_serialization() {
        let req = DapRequest::new(1, "initialize", None);
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"seq\":1"));
        assert!(json.contains("\"command\":\"initialize\""));
    }

    #[test]
    fn test_initialize_arguments() {
        let args = InitializeArguments {
            adapter_id: "rust".to_string(),
            ..Default::default()
        };

        assert_eq!(args.client_id, Some("claude-visual".to_string()));
        assert!(args.lines_start_at1);
    }

    #[test]
    fn test_breakpoint() {
        let bp = Breakpoint {
            id: Some(1),
            verified: true,
            message: None,
            source: Some(Source {
                name: Some("main.rs".to_string()),
                path: Some("/src/main.rs".to_string()),
                source_reference: None,
                presentation_hint: None,
                origin: None,
            }),
            line: Some(42),
            column: None,
            end_line: None,
            end_column: None,
            instruction_reference: None,
            offset: None,
        };

        assert!(bp.verified);
        assert_eq!(bp.line, Some(42));
    }
}
