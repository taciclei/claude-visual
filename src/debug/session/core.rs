//! Core debug session structure and basic operations

use std::collections::HashMap;
use std::path::PathBuf;

use tokio::sync::mpsc;

use super::super::client::{DapClient, DapClientConfig};
use super::super::protocol::*;
use super::types::*;

/// Debug session manager
pub struct DebugSession {
    /// DAP client
    pub(crate) client: DapClient,
    /// Current state
    pub(crate) state: DebugState,
    /// Event receiver from client
    pub(crate) event_receiver: Option<mpsc::UnboundedReceiver<DapEvent>>,
    /// Session event sender
    pub(crate) session_event_sender: mpsc::UnboundedSender<SessionEvent>,
    /// User-defined breakpoints
    pub(crate) breakpoints: HashMap<usize, UserBreakpoint>,
    /// Next breakpoint ID
    pub(crate) next_breakpoint_id: usize,
    /// Current thread ID
    pub(crate) current_thread_id: Option<i64>,
    /// Current frame ID
    pub(crate) current_frame_id: Option<i64>,
    /// Threads
    pub(crate) threads: Vec<Thread>,
    /// Stack frames for current thread
    pub(crate) stack_frames: Vec<StackFrame>,
    /// Variables cache
    pub(crate) variables_cache: HashMap<i64, Vec<Variable>>,
    /// Adapter capabilities
    pub(crate) capabilities: Option<Capabilities>,
    /// Launch/attach configuration
    pub(crate) launch_config: Option<LaunchArguments>,
}

impl DebugSession {
    /// Create a new debug session
    pub fn new(config: DapClientConfig) -> (Self, mpsc::UnboundedReceiver<SessionEvent>) {
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        let session = Self {
            client: DapClient::new(config),
            state: DebugState::Idle,
            event_receiver: None,
            session_event_sender: event_tx,
            breakpoints: HashMap::new(),
            next_breakpoint_id: 1,
            current_thread_id: None,
            current_frame_id: None,
            threads: Vec::new(),
            stack_frames: Vec::new(),
            variables_cache: HashMap::new(),
            capabilities: None,
            launch_config: None,
        };

        (session, event_rx)
    }

    /// Get current state
    pub fn state(&self) -> DebugState {
        self.state
    }

    /// Set state and emit event
    pub(crate) fn set_state(&mut self, state: DebugState) {
        self.state = state;
        let _ = self.session_event_sender.send(SessionEvent::StateChanged(state));
    }

    /// Get breakpoints for a file
    pub fn breakpoints_for_file(&self, file: &PathBuf) -> Vec<&UserBreakpoint> {
        self.breakpoints
            .values()
            .filter(|bp| &bp.file == file)
            .collect()
    }

    /// Get current thread ID
    pub fn current_thread_id(&self) -> Option<i64> {
        self.current_thread_id
    }

    /// Get current frame ID
    pub fn current_frame_id(&self) -> Option<i64> {
        self.current_frame_id
    }

    /// Set current thread
    pub fn set_current_thread(&mut self, thread_id: i64) {
        self.current_thread_id = Some(thread_id);
    }

    /// Set current frame
    pub fn set_current_frame(&mut self, frame_id: i64) {
        self.current_frame_id = Some(frame_id);
    }
}
