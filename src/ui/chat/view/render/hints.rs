//! Contextual hints render functions for ChatView

use gpui::*;
use gpui::prelude::*;
use super::super::core::ChatView;
use super::super::types::ChatViewEvent;

impl ChatView {
    pub fn render_quick_reply_suggestions(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> Div {
        // Clone suggestions to avoid lifetime issues
        let suggestions: Vec<_> = self.quick_reply_suggestions.iter().cloned().collect();

        div()
            .flex()
            .items_center()
            .gap_2()
            .px_4()
            .py_2()
            .bg(theme.colors.surface.opacity(0.5))
            .border_t_1()
            .border_color(theme.colors.border.opacity(0.3))
            // Label
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted.opacity(0.7))
                    .child("Quick replies:")
            )
            // Suggestions
            .children(
                suggestions.into_iter().enumerate().map(|(idx, suggestion)| {
                    let prompt = suggestion.prompt.clone();
                    let icon = suggestion.icon.to_string();
                    let label = suggestion.label.clone();
                    div()
                        .id(ElementId::Name(format!("quick-reply-{}", idx).into()))
                        .flex()
                        .items_center()
                        .gap_1()
                        .px_2()
                        .py_1()
                        .rounded_md()
                        .cursor_pointer()
                        .bg(theme.colors.background)
                        .border_1()
                        .border_color(theme.colors.border.opacity(0.5))
                        .text_xs()
                        .text_color(theme.colors.text_muted)
                        .hover(|s| s.bg(theme.colors.surface_hover).text_color(theme.colors.text).border_color(theme.colors.accent.opacity(0.5)))
                        .on_click(cx.listener(move |this, _, _window, cx| {
                            this.input.update(cx, |input, cx| {
                                input.set_text(prompt.clone(), cx);
                            });
                            cx.emit(ChatViewEvent::Submit(prompt.clone()));
                        }))
                        .child(icon)
                        .child(label)
                })
            )
    }


    pub fn render_contextual_quick_actions(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> Div {
        let actions = self.get_quick_actions();

        if actions.is_empty() {
            return div();
        }

        div()
            .flex()
            .items_center()
            .gap_2()
            .px_4()
            .py_2()
            .bg(theme.colors.surface.opacity(0.3))
            // Label
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted.opacity(0.7))
                    .child("Quick actions:")
            )
            // Actions
            .children(
                actions.into_iter().map(|(icon, label, action)| {
                    let action_str = action.to_string();
                    div()
                        .id(ElementId::Name(format!("quick-action-{}", label).into()))
                        .flex()
                        .items_center()
                        .gap_1()
                        .px_3()
                        .py_1()
                        .rounded_md()
                        .cursor_pointer()
                        .bg(theme.colors.accent.opacity(0.1))
                        .border_1()
                        .border_color(theme.colors.accent.opacity(0.2))
                        .text_xs()
                        .text_color(theme.colors.accent)
                        .hover(|s| s.bg(theme.colors.accent.opacity(0.2)).border_color(theme.colors.accent.opacity(0.4)))
                        .on_click(cx.listener(move |this, _, _window, cx| {
                            if action_str == "retry" {
                                this.retry_last_request(cx);
                            } else if action_str.starts_with('/') {
                                cx.emit(ChatViewEvent::Submit(action_str.clone()));
                            } else {
                                cx.emit(ChatViewEvent::Submit(action_str.clone()));
                            }
                        }))
                        .child(icon)
                        .child(label)
                })
            )
    }


    pub fn render_contextual_hints(&self, theme: &crate::app::theme::Theme) -> Div {
        // Collect hints based on current state
        let mut hints: Vec<(&'static str, &'static str)> = Vec::new();

        // Always show escape hint during streaming
        if self.streaming.is_streaming {
            hints.push(("⌘.", "Stop"));
            // Show current tool hint
            if self.current_tool_name.is_some() {
                hints.push(("⏳", "Tool running"));
            }
        }

        // Show retry hint after error
        if self.last_error.is_some() {
            hints.push(("⌘R", "Retry"));
            hints.push(("/debug", "Debug"));
        }

        // Show continue hint if truncated
        if self.is_last_response_truncated() {
            hints.push(("⌘↩", "Continue"));
        }

        // Git-related hints
        if let Some(ref git) = self.git_info {
            if git.staged_count > 0 {
                hints.push(("/commit", "Commit"));
                hints.push(("/create-pr", "PR"));
            }
            if git.is_dirty {
                hints.push(("/review", "Review"));
            }
        }

        // Context usage hints with different levels
        let ctx_usage = self.context_usage_percentage();
        if ctx_usage > 0.9 {
            hints.push(("/compact", "COMPACT!"));
        } else if ctx_usage > 0.7 {
            hints.push(("/compact", "Compact"));
        }

        // Code-related hints based on last message
        if let Some(last_msg) = self.messages.last() {
            if last_msg.content.contains("```") {
                hints.push(("/review-code", "Review"));
                hints.push(("/explain", "Explain"));
            }
            if last_msg.content.to_lowercase().contains("error") {
                hints.push(("/debug", "Debug"));
            }
        }

        // Show search hint when messages exist
        if self.messages.len() > 5 {
            hints.push(("⌘F", "Search"));
        }

        // Session management hints
        if self.messages.len() > 20 {
            hints.push(("/memory", "Memory"));
        }

        // Always available hints
        hints.push(("⌘K", "Commands"));
        hints.push(("/", "Skills"));

        // Only show if we have hints
        if hints.is_empty() {
            return div();
        }

        div()
            .flex()
            .items_center()
            .gap_4()
            .px_4()
            .py_1()
            .bg(theme.colors.surface.opacity(0.3))
            .border_t_1()
            .border_color(theme.colors.border.opacity(0.2))
            .children(
                hints.into_iter().map(|(shortcut, label)| {
                    // Highlight urgent hints
                    let is_urgent = label == "COMPACT!" || label == "Debug";
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .text_xs()
                        .child(
                            div()
                                .px_1()
                                .py(px(1.0))
                                .rounded_sm()
                                .bg(if is_urgent { theme.colors.warning.opacity(0.2) } else { theme.colors.surface })
                                .border_1()
                                .border_color(if is_urgent { theme.colors.warning.opacity(0.5) } else { theme.colors.border.opacity(0.5) })
                                .font_family("monospace")
                                .text_color(if is_urgent { theme.colors.warning } else { theme.colors.text_muted })
                                .child(shortcut)
                        )
                        .child(
                            div()
                                .text_color(if is_urgent { theme.colors.warning } else { theme.colors.text_muted.opacity(0.7) })
                                .child(label)
                        )
                })
            )
    }
}
