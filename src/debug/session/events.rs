//! Event processing from DAP adapter

use super::super::protocol::*;
use super::core::DebugSession;
use super::types::{DebugState, SessionEvent};

impl DebugSession {
    /// Process events from adapter
    pub async fn process_events(&mut self) {
        // Collect all pending events first to avoid borrow conflicts
        let mut events = Vec::new();
        if let Some(receiver) = &mut self.event_receiver {
            while let Ok(event) = receiver.try_recv() {
                events.push(event);
            }
        }

        // Handle all collected events
        for event in events {
            handle_event(self, event).await;
        }
    }
}

/// Handle a single event from adapter (internal helper)
async fn handle_event(session: &mut DebugSession, event: DapEvent) {
    match event.event.as_str() {
        "stopped" => {
            if let Some(body) = event.body {
                if let Ok(stopped) = serde_json::from_value::<StoppedEventBody>(body) {
                    session.set_state(DebugState::Stopped);
                    session.current_thread_id = stopped.thread_id;

                    let _ = session.session_event_sender.send(SessionEvent::Stopped {
                        reason: stopped.reason,
                        thread_id: stopped.thread_id,
                        description: stopped.description,
                    });

                    // Refresh stack trace
                    let _ = session.refresh_stack_trace().await;
                }
            }
        }
        "continued" => {
            session.set_state(DebugState::Running);
        }
        "output" => {
            if let Some(body) = event.body {
                if let Ok(output) = serde_json::from_value::<OutputEventBody>(body) {
                    let _ = session.session_event_sender.send(SessionEvent::Output {
                        category: output.category.unwrap_or_else(|| "console".to_string()),
                        text: output.output,
                    });
                }
            }
        }
        "terminated" => {
            session.set_state(DebugState::Terminated);
            let _ = session.session_event_sender.send(SessionEvent::Terminated);
        }
        "thread" => {
            if let Some(body) = event.body {
                let thread_id = body.get("threadId").and_then(|v| v.as_i64()).unwrap_or(0);
                let reason = body.get("reason").and_then(|v| v.as_str()).unwrap_or("");

                match reason {
                    "started" => {
                        let _ = session
                            .session_event_sender
                            .send(SessionEvent::ThreadStarted(thread_id));
                    }
                    "exited" => {
                        let _ = session
                            .session_event_sender
                            .send(SessionEvent::ThreadExited(thread_id));
                    }
                    _ => {}
                }
            }
        }
        "breakpoint" => {
            if let Some(body) = event.body {
                if let Some(bp) = body.get("breakpoint") {
                    if let Ok(breakpoint) = serde_json::from_value::<Breakpoint>(bp.clone()) {
                        let _ = session
                            .session_event_sender
                            .send(SessionEvent::BreakpointChanged(breakpoint));
                    }
                }
            }
        }
        "module" => {
            if let Some(body) = event.body {
                if let Some(module) = body.get("module") {
                    let name = module
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let path = module
                        .get("path")
                        .and_then(|v| v.as_str())
                        .map(String::from);

                    let _ = session
                        .session_event_sender
                        .send(SessionEvent::ModuleLoaded { name, path });
                }
            }
        }
        _ => {
            // Unknown event
        }
    }
}
