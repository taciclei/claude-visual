//! Debug Adapter Protocol (DAP) Module
//!
//! Client implementation for the Debug Adapter Protocol,
//! enabling debugging support for multiple languages.

mod client;
mod protocol;
mod session;

pub use client::{DapClient, DapClientConfig, DapClientError};
pub use protocol::{
    AttachArguments, Breakpoint, BreakpointLocation, Capabilities, DapEvent, DapMessage,
    DapRequest, DapResponse, ExceptionBreakpointsFilter, InitializeArguments, LaunchArguments,
    OutputEventBody, Scope, Source, StackFrame, StoppedEventBody, TerminatedEventBody, Thread,
    Variable,
};
pub use session::{DebugSession, DebugState, SessionEvent};
