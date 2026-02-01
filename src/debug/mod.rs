//! Debug Adapter Protocol (DAP) Module
//!
//! Client implementation for the Debug Adapter Protocol,
//! enabling debugging support for multiple languages.

mod protocol;
mod client;
mod session;

pub use protocol::{
    DapRequest, DapResponse, DapEvent, DapMessage,
    InitializeArguments, LaunchArguments, AttachArguments,
    Breakpoint, BreakpointLocation, Source, StackFrame, Scope, Variable,
    Thread, StoppedEventBody, OutputEventBody, TerminatedEventBody,
    Capabilities, ExceptionBreakpointsFilter,
};
pub use client::{DapClient, DapClientError, DapClientConfig};
pub use session::{DebugSession, DebugState, SessionEvent};
