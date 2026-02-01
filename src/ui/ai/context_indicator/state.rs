//! Context indicator state management

use gpui::*;
use super::colors::SimpleColors;
use super::usage::ContextUsage;
use super::events::ContextIndicatorEvent;

/// Context window indicator component
pub struct ContextIndicator {
    /// Current context usage
    pub(super) usage: ContextUsage,
    /// Whether to show detailed view
    pub(super) show_details: bool,
}

impl ContextIndicator {
    /// Create a new context indicator
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            usage: ContextUsage {
                current_tokens: 0,
                max_tokens: 200_000, // Claude default
                message_count: 0,
                file_count: 0,
                estimated_cost: None,
            },
            show_details: false,
        }
    }

    /// Update context usage
    pub fn update_usage(&mut self, usage: ContextUsage, cx: &mut Context<Self>) {
        self.usage = usage;
        cx.notify();
    }

    /// Set current tokens
    pub fn set_tokens(&mut self, tokens: usize, cx: &mut Context<Self>) {
        self.usage.current_tokens = tokens;
        cx.notify();
    }

    /// Set max tokens (model context window)
    pub fn set_max_tokens(&mut self, max: usize, cx: &mut Context<Self>) {
        self.usage.max_tokens = max;
        cx.notify();
    }

    /// Set message count
    pub fn set_message_count(&mut self, count: usize, cx: &mut Context<Self>) {
        self.usage.message_count = count;
        cx.notify();
    }

    /// Set file count
    pub fn set_file_count(&mut self, count: usize, cx: &mut Context<Self>) {
        self.usage.file_count = count;
        cx.notify();
    }

    /// Toggle details visibility
    pub fn toggle_details(&mut self, cx: &mut Context<Self>) {
        self.show_details = !self.show_details;
        cx.notify();
    }

    /// Get progress bar color based on usage
    pub(super) fn progress_color(&self, colors: &SimpleColors) -> Hsla {
        if self.usage.is_critical() {
            colors.error
        } else if self.usage.is_warning() {
            colors.warning
        } else {
            colors.success
        }
    }
}

impl EventEmitter<ContextIndicatorEvent> for ContextIndicator {}
