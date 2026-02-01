//! Error handling methods for ChatView

use gpui::*;
use crate::claude::message::MessageRole;
use super::types::*;
use super::core::ChatView;

impl ChatView {
    /// Record an error with context for recovery
    pub fn record_error(&mut self, message: String, original_prompt: Option<String>, cx: &mut Context<Self>) {
        let category = ErrorCategory::from_message(&message);
        self.last_error = Some(ErrorInfo {
            message,
            original_prompt,
            timestamp: chrono::Utc::now(),
            can_retry: true,
            category,
        });
        cx.notify();
    }

    /// Retry the last failed operation
    pub fn retry_last_error(&mut self, cx: &mut Context<Self>) {
        if let Some(error) = self.last_error.take() {
            if let Some(prompt) = error.original_prompt {
                cx.emit(ChatViewEvent::Submit(prompt));
            }
        }
        cx.notify();
    }

    /// Regenerate the last assistant response
    pub fn regenerate_last_response(&mut self, cx: &mut Context<Self>) {
        // Don't regenerate while streaming
        if self.streaming.is_streaming {
            self.show_notification("Cannot regenerate while streaming", NotificationType::Warning, cx);
            return;
        }

        // Find the last user message
        let last_user_prompt = self.messages.iter().rev()
            .find(|m| m.role == MessageRole::User)
            .map(|m| m.content.clone());

        if let Some(prompt) = last_user_prompt {
            // Remove the last assistant response (and any tool use/results after it)
            while let Some(last_msg) = self.messages.last() {
                if last_msg.role == MessageRole::User {
                    break;
                }
                self.messages.pop();
                self.message_views.pop();
            }

            // Resend the prompt
            cx.emit(ChatViewEvent::Submit(prompt));
            self.show_notification("Regenerating response...", NotificationType::Info, cx);
        } else {
            self.show_notification("No message to regenerate", NotificationType::Warning, cx);
        }
        cx.notify();
    }

    /// Stop the current streaming response (alias for request_stop)
    pub fn stop_streaming(&mut self, cx: &mut Context<Self>) {
        self.request_stop(cx);
    }

    /// Clear the last error
    pub fn clear_error(&mut self, cx: &mut Context<Self>) {
        self.last_error = None;
        cx.notify();
    }

    /// Retry the last failed request
    pub fn retry_last_request(&mut self, cx: &mut Context<Self>) {
        // Don't retry while streaming
        if self.streaming.is_streaming {
            self.show_notification("Cannot retry while streaming", NotificationType::Warning, cx);
            return;
        }

        // Check if we have an error with an original prompt
        if let Some(error_info) = &self.last_error {
            if let Some(prompt) = &error_info.original_prompt {
                let retry_prompt = prompt.clone();
                self.last_error = None;
                self.stats.connection_retry_count += 1;
                cx.emit(ChatViewEvent::Submit(retry_prompt));
                self.show_notification("Retrying request...", NotificationType::Info, cx);
                return;
            }
        }

        // If no error with prompt, try regenerating last response
        self.regenerate_last_response(cx);
    }

    /// Continue the conversation (ask Claude to continue from where it stopped)
    pub fn continue_conversation(&mut self, cx: &mut Context<Self>) {
        if self.streaming.is_streaming {
            self.show_notification("Cannot continue while streaming", NotificationType::Warning, cx);
            return;
        }

        // Send "continue" prompt
        cx.emit(ChatViewEvent::Submit("continue".to_string()));
        self.show_notification("Continuing conversation...", NotificationType::Info, cx);
    }

    /// Check if the last assistant message appears truncated
    pub fn is_last_response_truncated(&self) -> bool {
        if let Some(last_msg) = self.messages.iter().rev().find(|m| m.role == MessageRole::Assistant) {
            // Heuristics for truncation: ends mid-sentence, ends with ...
            let content = last_msg.content.trim();
            if content.is_empty() {
                return false;
            }
            // Check for common truncation patterns
            content.ends_with("...")
                || content.ends_with("…")
                || (content.len() > 100 && !content.ends_with('.') && !content.ends_with('!') && !content.ends_with('?') && !content.ends_with('`') && !content.ends_with('}'))
        } else {
            false
        }
    }

    /// Check if context is critical and needs compacting before proceeding
    /// Returns true if context is OK, false if critical (user should compact first)
    pub fn check_context_before_submit(&mut self, cx: &mut Context<Self>) -> bool {
        let usage = self.context_usage_percentage();

        if usage > 0.95 {
            // Critical - strongly recommend compacting
            self.show_notification(
                "Context is 95% full! Please use /compact before sending more messages to avoid context overflow.",
                NotificationType::Error,
                cx
            );
            return false;
        }

        if usage > 0.90 {
            // High - warn but allow
            self.show_notification(
                "Context is 90% full. Consider using /compact soon.",
                NotificationType::Warning,
                cx
            );
        }

        true
    }

    /// Smart submit that checks context before sending
    pub fn smart_submit(&mut self, text: String, cx: &mut Context<Self>) {
        // Check context before submitting
        if !self.check_context_before_submit(cx) {
            // Context is critical, but still allow the submit
            // User has been warned
        }

        cx.emit(ChatViewEvent::Submit(text));
    }
}
