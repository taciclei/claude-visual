//! Toast container component for managing notifications

use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;

use super::types::*;
use crate::app::state::AppState;

impl EventEmitter<ToastEvent> for ToastContainer {}

/// Container for managing multiple toasts
pub struct ToastContainer {
    app_state: Arc<AppState>,
    toasts: Vec<Toast>,
    next_id: usize,
}

impl ToastContainer {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            toasts: Vec::new(),
            next_id: 1,
        }
    }

    /// Show a new toast notification
    pub fn show(&mut self, mut toast: Toast, cx: &mut Context<Self>) {
        toast.id = self.next_id;
        self.next_id += 1;

        let id = toast.id;
        let duration = toast.duration;

        self.toasts.push(toast);
        cx.notify();

        // Schedule auto-dismiss if duration is set
        if let Some(duration) = duration {
            cx.spawn(async move |this, cx| {
                cx.background_executor().timer(duration).await;
                let _ = this.update(cx, |this, cx| {
                    this.dismiss(id, cx);
                });
            })
            .detach();
        }
    }

    /// Dismiss a toast by ID
    pub fn dismiss(&mut self, id: usize, cx: &mut Context<Self>) {
        self.toasts.retain(|t| t.id != id);
        cx.emit(ToastEvent::Dismissed(id));
        cx.notify();
    }

    /// Clear all toasts
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.toasts.clear();
        cx.notify();
    }

    /// Get the number of active toasts
    pub fn count(&self) -> usize {
        self.toasts.len()
    }
}

impl Render for ToastContainer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        // Container for toasts - positioned at bottom right
        div()
            .id("toast-container")
            .absolute()
            .bottom_4()
            .right_4()
            .flex()
            .flex_col()
            .gap_2()
            .max_w(px(400.0))
            .children(self.toasts.iter().map(|toast| {
                let id = toast.id;
                let level = toast.level;
                let message = toast.message.clone();
                let action = toast.action.clone();
                let dismissible = toast.dismissible;

                // Get colors based on level
                let (bg_color, border_color, icon_color) = match level {
                    ToastLevel::Info => (
                        theme.colors.info.opacity(0.15),
                        theme.colors.info.opacity(0.5),
                        theme.colors.info,
                    ),
                    ToastLevel::Success => (
                        theme.colors.success.opacity(0.15),
                        theme.colors.success.opacity(0.5),
                        theme.colors.success,
                    ),
                    ToastLevel::Warning => (
                        theme.colors.warning.opacity(0.15),
                        theme.colors.warning.opacity(0.5),
                        theme.colors.warning,
                    ),
                    ToastLevel::Error => (
                        theme.colors.error.opacity(0.15),
                        theme.colors.error.opacity(0.5),
                        theme.colors.error,
                    ),
                };

                div()
                    .id(SharedString::from(format!("toast-{}", id)))
                    .px_4()
                    .py_3()
                    .rounded_lg()
                    .bg(theme.colors.surface)
                    .border_1()
                    .border_color(border_color)
                    .shadow_lg()
                    .flex()
                    .items_center()
                    .gap_3()
                    // Icon
                    .child(
                        div()
                            .size(px(24.0))
                            .rounded_full()
                            .bg(bg_color)
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .text_color(icon_color)
                            .child(level.icon()),
                    )
                    // Message
                    .child(
                        div()
                            .flex_1()
                            .text_sm()
                            .text_color(theme.colors.text)
                            .child(message),
                    )
                    // Action button (if any)
                    .when_some(action, |this, action_text| {
                        this.child(
                            div()
                                .id(SharedString::from(format!("toast-action-{}", id)))
                                .px_2()
                                .py_1()
                                .rounded_md()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.colors.accent)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.colors.accent.opacity(0.1)))
                                .on_click(cx.listener(move |this, _, _window, cx| {
                                    cx.emit(ToastEvent::ActionClicked(id));
                                    this.dismiss(id, cx);
                                }))
                                .child(action_text),
                        )
                    })
                    // Dismiss button (if dismissible)
                    .when(dismissible, |this| {
                        this.child(
                            div()
                                .id(SharedString::from(format!("toast-dismiss-{}", id)))
                                .size(px(20.0))
                                .rounded_md()
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .cursor_pointer()
                                .hover(|s| {
                                    s.bg(theme.colors.surface_hover)
                                        .text_color(theme.colors.text)
                                })
                                .on_click(cx.listener(move |this, _, _window, cx| {
                                    this.dismiss(id, cx);
                                }))
                                .child("x"),
                        )
                    })
            }))
    }
}
