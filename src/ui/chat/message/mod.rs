//! Message bubble component with collapse/expand functionality

use gpui::prelude::*;
use gpui::*;

use crate::claude::message::MessageRole;

// Module declarations
pub mod render;
pub mod types;
pub mod utils;
pub mod view;

#[cfg(test)]
mod tests;

// Re-exports for public API
pub use types::{MessageAction, MessageReaction, MessageViewEvent};
pub use utils::format_relative_time;
pub use view::MessageView;

// Implement EventEmitter for MessageView
impl EventEmitter<MessageViewEvent> for MessageView {}

// Implement Render for MessageView
impl Render for MessageView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_clone = {
            let theme = self.app_state.theme.read(cx);
            theme.clone()
        };
        let theme = &theme_clone;
        let collapsed = self.collapsed;
        let is_error = self.message.is_error;
        let show_menu = self.show_context_menu;
        let is_selected = self.selected;

        // Choose border color based on role
        let border_color = match self.message.role {
            MessageRole::User => theme.colors.accent,
            MessageRole::Assistant => theme.colors.border,
            MessageRole::ToolUse => theme.colors.info,
            MessageRole::ToolResult if is_error => theme.colors.error,
            MessageRole::ToolResult => theme.colors.success,
            MessageRole::Error => theme.colors.error,
            MessageRole::Thinking => theme.colors.warning.opacity(0.7),
            MessageRole::System => theme.colors.text_muted,
        };

        // Selection highlight color
        let bg_color = if is_selected {
            theme.colors.accent.opacity(0.1)
        } else {
            theme.colors.surface
        };

        div()
            .relative()
            .id(ElementId::Name(
                format!("msg-{}", self.message.timestamp.timestamp_millis()).into(),
            ))
            .w_full()
            .rounded_lg()
            .bg(bg_color)
            .border_l_2()
            .border_color(border_color)
            // Add accent border when selected
            .when(is_selected, |d| {
                d.border_1().border_color(theme.colors.accent.opacity(0.3))
            })
            // Right-click to show context menu
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(|this, event: &MouseDownEvent, _window, cx| {
                    this.show_context_menu(event.position, cx);
                }),
            )
            // Click outside context menu to close it
            .when(show_menu, |d| {
                d.on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, _window, cx| {
                        this.hide_context_menu(cx);
                    }),
                )
            })
            // Enable group for hover toolbar
            .group("message")
            // Hover toolbar (visible on hover via CSS)
            .when(!show_menu && !collapsed, |d| {
                d.child(self.render_hover_toolbar(&theme, cx))
            })
            // Header (always visible, clickable to collapse)
            .child(
                div()
                    .id("message-header")
                    .cursor_pointer()
                    .hover(|style| style.bg(theme.colors.surface_hover))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.toggle_collapsed(cx);
                    }))
                    .child(self.render_header(&theme)),
            )
            // Content (collapsible)
            .when(!collapsed, |this| {
                this.child(match self.message.role {
                    MessageRole::User => self.render_user_content(&theme),
                    MessageRole::Assistant => self.render_assistant_content(&theme),
                    MessageRole::ToolUse | MessageRole::ToolResult => {
                        self.render_tool_content(&theme, cx)
                    }
                    MessageRole::Error => self.render_error_content(&theme, cx),
                    MessageRole::Thinking => self.render_thinking_content(&theme),
                    MessageRole::System => self.render_system_content(&theme),
                })
            })
            // Actions footer (visible on hover or when expanded)
            .when(!collapsed, |this| {
                let can_react = self.can_react();
                let current_reaction = self.reaction;

                this.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .px_3()
                        .py_1()
                        .border_t_1()
                        .border_color(theme.colors.border)
                        // Left side - Reactions (only for assistant messages)
                        .child(div().flex().items_center().gap_1().when(can_react, |d| {
                            d
                                // Thumbs up
                                .child(
                                    div()
                                        .id("reaction-up")
                                        .px_2()
                                        .py_1()
                                        .rounded_sm()
                                        .cursor_pointer()
                                        .bg(
                                            if current_reaction == Some(MessageReaction::ThumbsUp) {
                                                theme.colors.success.opacity(0.2)
                                            } else {
                                                gpui::transparent_black()
                                            },
                                        )
                                        .hover(|s| s.bg(theme.colors.surface_hover))
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.set_reaction(MessageReaction::ThumbsUp, cx);
                                        }))
                                        .child("👍"),
                                )
                                // Thumbs down
                                .child(
                                    div()
                                        .id("reaction-down")
                                        .px_2()
                                        .py_1()
                                        .rounded_sm()
                                        .cursor_pointer()
                                        .bg(
                                            if current_reaction == Some(MessageReaction::ThumbsDown)
                                            {
                                                theme.colors.error.opacity(0.2)
                                            } else {
                                                gpui::transparent_black()
                                            },
                                        )
                                        .hover(|s| s.bg(theme.colors.surface_hover))
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.set_reaction(MessageReaction::ThumbsDown, cx);
                                        }))
                                        .child("👎"),
                                )
                        }))
                        // Right side - Actions
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                // Copy button
                                .child(
                                    div()
                                        .id("copy-message")
                                        .px_2()
                                        .py_1()
                                        .rounded_sm()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .hover(|style| {
                                            style
                                                .bg(theme.colors.surface_hover)
                                                .text_color(theme.colors.text)
                                        })
                                        .cursor_pointer()
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.copy_to_clipboard(cx);
                                        }))
                                        .child("Copy"),
                                )
                                // Bookmark button
                                .child({
                                    let is_bookmarked = self.bookmarked;
                                    div()
                                        .id("bookmark-message")
                                        .px_2()
                                        .py_1()
                                        .rounded_sm()
                                        .text_xs()
                                        .text_color(if is_bookmarked {
                                            theme.colors.warning
                                        } else {
                                            theme.colors.text_muted
                                        })
                                        .bg(if is_bookmarked {
                                            theme.colors.warning.opacity(0.1)
                                        } else {
                                            gpui::transparent_black()
                                        })
                                        .hover(|style| {
                                            style
                                                .bg(theme.colors.surface_hover)
                                                .text_color(theme.colors.warning)
                                        })
                                        .cursor_pointer()
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.toggle_bookmark(cx);
                                        }))
                                        .child(if is_bookmarked { "★" } else { "☆" })
                                })
                                // More actions button
                                .child(self.render_more_button(&theme, cx)),
                        ),
                )
            })
            // Context menu overlay
            .when(show_menu, |this| {
                this.child(self.render_context_menu(&theme, cx))
            })
    }
}
