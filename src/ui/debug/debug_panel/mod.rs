//! Debug Panel
//!
//! Main debug control panel with toolbar and session status.

use std::sync::Arc;

use gpui::*;

use crate::app::state::AppState;
use crate::debug::{DebugState, SessionEvent};

mod types;
mod render;

pub use types::{DebugContext, DebugPanelEvent, DebugPromptType, DebugTab};
use types::OutputLine;

impl EventEmitter<DebugPanelEvent> for DebugPanel {}

/// Debug panel component
pub struct DebugPanel {
    pub(super) app_state: Arc<AppState>,
    /// Current debug state
    pub(super) state: DebugState,
    /// Session output
    pub(super) output: Vec<OutputLine>,
    /// Is panel expanded
    pub(super) expanded: bool,
    /// Active tab
    pub(super) active_tab: DebugTab,
    /// Current thread name
    pub(super) current_thread: Option<String>,
    /// Current file/line
    pub(super) current_location: Option<(String, i64)>,
    /// Show AI prompts menu
    pub(super) show_ai_menu: bool,
    /// Last error message
    pub(super) last_error: Option<String>,
    /// Stack frames
    pub(super) stack_frames: Vec<String>,
    /// Current variables
    pub(super) variables: Vec<(String, String)>,
}

impl DebugPanel {
    /// Create a new debug panel
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            state: DebugState::Idle,
            output: Vec::new(),
            expanded: true,
            active_tab: DebugTab::Console,
            current_thread: None,
            current_location: None,
            show_ai_menu: false,
            last_error: None,
            stack_frames: Vec::new(),
            variables: Vec::new(),
        }
    }

    /// Toggle AI prompts menu
    pub fn toggle_ai_menu(&mut self, cx: &mut Context<Self>) {
        self.show_ai_menu = !self.show_ai_menu;
        cx.notify();
    }

    /// Close AI prompts menu
    pub fn close_ai_menu(&mut self, cx: &mut Context<Self>) {
        self.show_ai_menu = false;
        cx.notify();
    }

    /// Get current debug context for AI prompts
    pub fn get_debug_context(&self) -> DebugContext {
        DebugContext {
            state: self.state,
            current_location: self.current_location.clone(),
            current_thread: self.current_thread.clone(),
            last_error: self.last_error.clone(),
            console_output: self.output.iter().map(|l| l.text.clone()).collect(),
            stack_frames: self.stack_frames.clone(),
            variables: self.variables.clone(),
        }
    }

    /// Set stack frames
    pub fn set_stack_frames(&mut self, frames: Vec<String>, cx: &mut Context<Self>) {
        self.stack_frames = frames;
        cx.notify();
    }

    /// Set variables
    pub fn set_variables(&mut self, vars: Vec<(String, String)>, cx: &mut Context<Self>) {
        self.variables = vars;
        cx.notify();
    }

    /// Update state
    pub fn set_state(&mut self, state: DebugState, cx: &mut Context<Self>) {
        self.state = state;
        cx.notify();
    }

    /// Handle session event
    pub fn handle_event(&mut self, event: SessionEvent, cx: &mut Context<Self>) {
        match event {
            SessionEvent::StateChanged(state) => {
                self.state = state;
            }
            SessionEvent::Output { category, text } => {
                use std::time::{SystemTime, UNIX_EPOCH};
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);

                self.output.push(OutputLine {
                    category,
                    text,
                    timestamp,
                });

                // Keep last 1000 lines
                if self.output.len() > 1000 {
                    self.output.remove(0);
                }
            }
            SessionEvent::Stopped { reason, thread_id, description } => {
                self.state = DebugState::Stopped;
                if let Some(desc) = description {
                    self.output.push(OutputLine {
                        category: "debug".to_string(),
                        text: format!("Stopped: {} (thread {})", desc, thread_id.unwrap_or(0)),
                        timestamp: 0,
                    });
                }
                let _ = reason; // Use reason if needed
            }
            SessionEvent::Terminated => {
                self.state = DebugState::Terminated;
            }
            SessionEvent::Error(msg) => {
                self.last_error = Some(msg.clone());
                self.output.push(OutputLine {
                    category: "stderr".to_string(),
                    text: format!("Error: {}", msg),
                    timestamp: 0,
                });
            }
            _ => {}
        }
        cx.notify();
    }

    /// Set current location
    pub fn set_location(&mut self, file: String, line: i64, cx: &mut Context<Self>) {
        self.current_location = Some((file, line));
        cx.notify();
    }

    /// Clear output
    pub fn clear_output(&mut self, cx: &mut Context<Self>) {
        self.output.clear();
        cx.notify();
    }

    /// Toggle expanded
    pub fn toggle_expanded(&mut self, cx: &mut Context<Self>) {
        self.expanded = !self.expanded;
        cx.notify();
    }

    /// Set active tab
    pub fn set_tab(&mut self, tab: DebugTab, cx: &mut Context<Self>) {
        self.active_tab = tab;
        cx.notify();
    }
}
