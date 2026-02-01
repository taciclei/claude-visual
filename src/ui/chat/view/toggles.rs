//! UI toggle and display settings methods for ChatView

use gpui::Context;
use crate::claude::message::MessageRole;
use super::core::ChatView;
use super::types::NotificationType;

impl ChatView {
    /// Toggle the stats bar visibility
    pub(crate) fn toggle_stats(&mut self, cx: &mut Context<Self>) {
        self.show_stats = !self.show_stats;
        cx.notify();
    }

    /// Show or hide stats bar
    pub(crate) fn set_show_stats(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_stats = show;
        cx.notify();
    }

    /// Collapse all messages
    pub(crate) fn collapse_all(&mut self, cx: &mut Context<Self>) {
        for message_view in &self.message_views {
            message_view.update(cx, |view, cx| {
                view.set_collapsed(true, cx);
            });
        }
        cx.notify();
    }

    /// Expand all messages
    pub(crate) fn expand_all(&mut self, cx: &mut Context<Self>) {
        for message_view in &self.message_views {
            message_view.update(cx, |view, cx| {
                view.set_collapsed(false, cx);
            });
        }
        cx.notify();
    }

    /// Check if all messages are collapsed
    pub(crate) fn are_all_collapsed(&self, cx: &Context<Self>) -> bool {
        if self.message_views.is_empty() {
            return false;
        }
        self.message_views.iter().all(|v| v.read(cx).is_collapsed())
    }

    /// Check if all messages are expanded
    pub(crate) fn are_all_expanded(&self, cx: &Context<Self>) -> bool {
        if self.message_views.is_empty() {
            return true;
        }
        self.message_views.iter().all(|v| !v.read(cx).is_collapsed())
    }

    /// Collapse tool messages only (tool_use and tool_result)
    pub(crate) fn collapse_tool_messages(&mut self, cx: &mut Context<Self>) {
        for (view, msg) in self.message_views.iter().zip(self.messages.iter()) {
            if matches!(msg.role, MessageRole::ToolUse | MessageRole::ToolResult) {
                view.update(cx, |v, cx| v.set_collapsed(true, cx));
            }
        }
        self.show_notification("Tool messages collapsed", NotificationType::Info, cx);
        cx.notify();
    }

    /// Expand tool messages only
    pub(crate) fn expand_tool_messages(&mut self, cx: &mut Context<Self>) {
        for (view, msg) in self.message_views.iter().zip(self.messages.iter()) {
            if matches!(msg.role, MessageRole::ToolUse | MessageRole::ToolResult) {
                view.update(cx, |v, cx| v.set_collapsed(false, cx));
            }
        }
        self.show_notification("Tool messages expanded", NotificationType::Info, cx);
        cx.notify();
    }

    /// Toggle collapse state of tool messages
    pub(crate) fn toggle_collapse_tool_messages(&mut self, cx: &mut Context<Self>) {
        // Check if any tool message is expanded
        let any_expanded = self.message_views.iter()
            .zip(self.messages.iter())
            .any(|(view, msg)| {
                matches!(msg.role, MessageRole::ToolUse | MessageRole::ToolResult) && !view.read(cx).is_collapsed()
            });

        if any_expanded {
            self.collapse_tool_messages(cx);
        } else {
            self.expand_tool_messages(cx);
        }
    }

    /// Collapse assistant messages only
    pub(crate) fn collapse_assistant_messages(&mut self, cx: &mut Context<Self>) {
        for (view, msg) in self.message_views.iter().zip(self.messages.iter()) {
            if matches!(msg.role, MessageRole::Assistant) {
                view.update(cx, |v, cx| v.set_collapsed(true, cx));
            }
        }
        self.show_notification("Assistant messages collapsed", NotificationType::Info, cx);
        cx.notify();
    }

    /// Expand assistant messages only
    pub(crate) fn expand_assistant_messages(&mut self, cx: &mut Context<Self>) {
        for (view, msg) in self.message_views.iter().zip(self.messages.iter()) {
            if matches!(msg.role, MessageRole::Assistant) {
                view.update(cx, |v, cx| v.set_collapsed(false, cx));
            }
        }
        self.show_notification("Assistant messages expanded", NotificationType::Info, cx);
        cx.notify();
    }

    /// Count of tool messages
    pub(crate) fn tool_message_count(&self) -> usize {
        self.messages.iter()
            .filter(|m| matches!(m.role, MessageRole::ToolUse | MessageRole::ToolResult))
            .count()
    }

    /// Toggle timestamp visibility
    pub(crate) fn toggle_timestamps(&mut self, cx: &mut Context<Self>) {
        self.show_timestamps = !self.show_timestamps;
        cx.notify();
    }

    /// Set timestamp visibility
    pub(crate) fn set_show_timestamps(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_timestamps = show;
        cx.notify();
    }

    /// Check if timestamps are visible
    pub(crate) fn timestamps_visible(&self) -> bool {
        self.show_timestamps
    }

    /// Toggle compact mode
    pub(crate) fn toggle_compact_mode(&mut self, cx: &mut Context<Self>) {
        self.compact_mode = !self.compact_mode;
        cx.notify();
    }

    /// Set compact mode
    pub(crate) fn set_compact_mode(&mut self, compact: bool, cx: &mut Context<Self>) {
        self.compact_mode = compact;
        cx.notify();
    }

    /// Check if compact mode is enabled
    pub(crate) fn is_compact_mode(&self) -> bool {
        self.compact_mode
    }

    /// Toggle time separators
    pub(crate) fn toggle_time_separators(&mut self, cx: &mut Context<Self>) {
        self.show_time_separators = !self.show_time_separators;
        cx.notify();
    }

    /// Check if time separators are enabled
    pub(crate) fn is_time_separators_enabled(&self) -> bool {
        self.show_time_separators
    }

    /// Toggle word wrap in code blocks
    pub(crate) fn toggle_word_wrap(&mut self, cx: &mut Context<Self>) {
        self.word_wrap = !self.word_wrap;
        cx.notify();
    }

    /// Check if word wrap is enabled
    pub(crate) fn is_word_wrap_enabled(&self) -> bool {
        self.word_wrap
    }

    /// Toggle line numbers in code blocks
    pub(crate) fn toggle_line_numbers(&mut self, cx: &mut Context<Self>) {
        self.show_line_numbers = !self.show_line_numbers;
        cx.notify();
    }

    /// Check if line numbers are shown
    pub(crate) fn is_line_numbers_enabled(&self) -> bool {
        self.show_line_numbers
    }

    /// Toggle focus mode (distraction-free input)
    pub(crate) fn toggle_focus_mode(&mut self, cx: &mut Context<Self>) {
        self.focus_mode = !self.focus_mode;
        if self.focus_mode {
            self.show_notification("Focus mode enabled".to_string(), NotificationType::Info, cx);
        } else {
            self.show_notification("Focus mode disabled".to_string(), NotificationType::Info, cx);
        }
        cx.notify();
    }

    /// Check if in focus mode
    pub(crate) fn is_focus_mode(&self) -> bool {
        self.focus_mode
    }

    /// Toggle history search panel (Ctrl+R)
    /// This shows a fuzzy search through input history
    pub(crate) fn toggle_history_search(&mut self, cx: &mut Context<Self>) {
        // For now, toggle search panel with a focus on history
        // A proper implementation would have a dedicated history search UI
        self.show_notification(
            "History search: Use ↑/↓ in input to browse history",
            NotificationType::Info,
            cx
        );
        // Focus the input to allow using arrow keys for history
        self.input.update(cx, |_input, cx| {
            cx.notify();
        });
        cx.notify();
    }
}
