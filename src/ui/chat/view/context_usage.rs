//! Context usage tracking and export methods

use gpui::*;
use crate::claude::message::MessageRole;
use super::core::ChatView;
use super::types::{ChatViewEvent, NotificationType, ExportFormat};

impl ChatView {
    /// Update context usage based on token counts
    pub fn update_context_usage(&mut self, used: u64, capacity: Option<u64>, cx: &mut Context<Self>) {
        let old_percentage = self.context_usage_percentage();

        self.context_used = used;
        if let Some(cap) = capacity {
            self.context_capacity = cap;
        }

        let new_percentage = self.context_usage_percentage();

        // Show proactive warnings when crossing thresholds
        if old_percentage <= 0.70 && new_percentage > 0.70 {
            self.show_notification(
                "Context is 70% full. Consider using /compact to free space.",
                NotificationType::Info,
                cx
            );
        } else if old_percentage <= 0.85 && new_percentage > 0.85 {
            self.show_notification(
                "Context is 85% full! Use /compact soon to avoid losing context.",
                NotificationType::Warning,
                cx
            );
        } else if old_percentage <= 0.95 && new_percentage > 0.95 {
            self.show_notification(
                "Context almost full! Use /compact immediately to prevent issues.",
                NotificationType::Error,
                cx
            );
        }

        // Recalculate session health when context changes significantly
        if (new_percentage - old_percentage).abs() > 0.05 {
            self.calculate_session_health(cx);
        }

        cx.notify();
    }

    /// Get context usage status color based on percentage
    pub(crate) fn context_status_color(&self, theme: &crate::app::theme::Theme) -> gpui::Hsla {
        let percentage = self.context_usage_percentage();
        if percentage > 0.95 {
            theme.colors.error
        } else if percentage > 0.80 {
            theme.colors.warning
        } else if percentage > 0.60 {
            theme.colors.info
        } else {
            theme.colors.success
        }
    }

    /// Get context usage warning message if needed
    pub(crate) fn context_warning_message(&self) -> Option<&'static str> {
        let percentage = self.context_usage_percentage();
        if percentage > 0.95 {
            Some("Context almost full! Consider using /compact")
        } else if percentage > 0.80 {
            Some("Context filling up. Consider compacting soon.")
        } else {
            None
        }
    }

    /// Format context usage for display
    pub fn format_context_usage(&self) -> String {
        let percentage = (self.context_usage_percentage() * 100.0) as u32;
        let used = if self.context_used >= 1000 {
            format!("{:.1}K", self.context_used as f64 / 1000.0)
        } else {
            self.context_used.to_string()
        };
        let capacity = if self.context_capacity >= 1000 {
            format!("{:.0}K", self.context_capacity as f64 / 1000.0)
        } else {
            self.context_capacity.to_string()
        };
        format!("{}/{}  ({}%)", used, capacity, percentage)
    }

    /// Get last response latency in milliseconds
    pub fn get_response_latency_ms(&self) -> Option<u64> {
        self.stats.last_response_latency_ms
    }

    /// Update response latency tracking
    pub fn update_response_latency(&mut self, latency_ms: u64, cx: &mut Context<Self>) {
        self.stats.last_response_latency_ms = Some(latency_ms);

        // Update rolling average (simple exponential moving average)
        if self.stats.avg_response_latency_ms == 0.0 {
            self.stats.avg_response_latency_ms = latency_ms as f64;
        } else {
            let alpha = 0.3; // Smoothing factor
            self.stats.avg_response_latency_ms = alpha * (latency_ms as f64) + (1.0 - alpha) * self.stats.avg_response_latency_ms;
        }

        // Recalculate session health
        self.calculate_session_health(cx);
    }

    /// Request to stop the current streaming response
    pub fn request_stop(&mut self, cx: &mut Context<Self>) {
        if self.streaming.is_streaming {
            cx.emit(ChatViewEvent::StopRequested);
        }
    }

    /// Request to export the conversation (shows export panel)
    pub fn request_export(&mut self, cx: &mut Context<Self>) {
        if self.has_messages() {
            self.panels.export_panel = true;
            cx.notify();
        } else {
            self.show_notification(
                "No messages to export".to_string(),
                NotificationType::Warning,
                cx
            );
        }
    }

    /// Request theme toggle (emits event for workspace to handle)
    pub fn request_theme_toggle(&mut self, cx: &mut Context<Self>) {
        cx.emit(ChatViewEvent::ThemeToggleRequested);
    }

    /// Toggle export panel visibility
    pub fn toggle_export_panel(&mut self, cx: &mut Context<Self>) {
        self.panels.export_panel = !self.panels.export_panel;
        cx.notify();
    }

    /// Set export format
    pub fn set_export_format(&mut self, format: ExportFormat, cx: &mut Context<Self>) {
        self.export.format = format;
        cx.notify();
    }

    /// Toggle include metadata option
    pub fn toggle_export_metadata(&mut self, cx: &mut Context<Self>) {
        self.export.include_metadata = !self.export.include_metadata;
        cx.notify();
    }

    /// Toggle include tools option
    pub fn toggle_export_tools(&mut self, cx: &mut Context<Self>) {
        self.export.include_tools = !self.export.include_tools;
        cx.notify();
    }

    /// Get the last assistant message content
    pub fn get_last_assistant_message(&self) -> Option<&str> {
        self.messages
            .iter()
            .rev()
            .find(|m| matches!(m.role, MessageRole::Assistant))
            .map(|m| m.content.as_str())
    }

    /// Get the last user message content
    pub fn get_last_user_message(&self) -> Option<&str> {
        self.messages
            .iter()
            .rev()
            .find(|m| matches!(m.role, MessageRole::User))
            .map(|m| m.content.as_str())
    }
}
