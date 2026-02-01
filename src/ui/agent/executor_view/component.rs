//! Executor view component definition

use gpui::*;
use crate::agent::executor::{ExecutorState, ExecutorStats};
use crate::app::theme::Theme;

/// Executor view component
pub struct ExecutorView {
    /// Current executor stats
    pub(super) stats: ExecutorStats,
    /// Current step description (if waiting for approval)
    pub(super) pending_approval: Option<String>,
    /// Theme
    pub(super) theme: Theme,
}

impl ExecutorView {
    /// Create a new executor view
    pub fn new(theme: Theme) -> Self {
        Self {
            stats: ExecutorStats {
                state: ExecutorState::Idle,
                completed_steps: 0,
                total_steps: 0,
                duration_ms: None,
            },
            pending_approval: None,
            theme,
        }
    }

    /// Update executor stats
    pub fn update_stats(&mut self, stats: ExecutorStats, cx: &mut Context<Self>) {
        self.stats = stats;
        cx.notify();
    }

    /// Set pending approval description
    pub fn set_pending_approval(&mut self, description: Option<String>, cx: &mut Context<Self>) {
        self.pending_approval = description;
        cx.notify();
    }

    /// Format duration
    pub(super) fn format_duration(ms: u64) -> String {
        let seconds = ms / 1000;
        let minutes = seconds / 60;
        let remaining_seconds = seconds % 60;

        if minutes > 0 {
            format!("{}m {}s", minutes, remaining_seconds)
        } else {
            format!("{}s", seconds)
        }
    }
}
