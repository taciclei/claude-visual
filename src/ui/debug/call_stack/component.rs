//! Call Stack View Component
//!
//! Component state and methods for call stack view.

use std::sync::Arc;

use gpui::*;

use super::types::{CallStackViewEvent, StackFrameItem, ThreadItem};
use crate::app::state::AppState;

impl EventEmitter<CallStackViewEvent> for CallStackView {}

/// Call stack view component
pub struct CallStackView {
    pub(super) app_state: Arc<AppState>,
    pub(super) threads: Vec<ThreadItem>,
    pub(super) current_thread_id: Option<i64>,
    pub(super) current_frame_id: Option<i64>,
}

impl CallStackView {
    /// Create a new call stack view
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            threads: Vec::new(),
            current_thread_id: None,
            current_frame_id: None,
        }
    }

    /// Set threads
    pub fn set_threads(&mut self, threads: Vec<ThreadItem>, cx: &mut Context<Self>) {
        self.threads = threads;
        cx.notify();
    }

    /// Set current thread and frame
    pub fn set_current(&mut self, thread_id: i64, frame_id: i64, cx: &mut Context<Self>) {
        self.current_thread_id = Some(thread_id);
        self.current_frame_id = Some(frame_id);

        // Mark current thread and frame
        for thread in &mut self.threads {
            thread.is_current = thread.id == thread_id;
            for frame in &mut thread.frames {
                frame.is_current = thread.id == thread_id && frame.id == frame_id;
            }
        }

        cx.notify();
    }

    /// Update frames for a thread
    pub fn set_frames(
        &mut self,
        thread_id: i64,
        frames: Vec<StackFrameItem>,
        cx: &mut Context<Self>,
    ) {
        if let Some(thread) = self.threads.iter_mut().find(|t| t.id == thread_id) {
            thread.frames = frames;
        }
        cx.notify();
    }

    /// Toggle thread expanded
    pub fn toggle_thread(&mut self, thread_id: i64, cx: &mut Context<Self>) {
        if let Some(thread) = self.threads.iter_mut().find(|t| t.id == thread_id) {
            thread.expanded = !thread.expanded;
        }
        cx.notify();
    }

    /// Clear all data
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.threads.clear();
        self.current_thread_id = None;
        self.current_frame_id = None;
        cx.notify();
    }
}
