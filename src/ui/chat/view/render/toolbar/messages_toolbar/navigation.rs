//! Navigation indicator render functions

use gpui::*;
use gpui::prelude::*;
use crate::claude::message::MessageRole;
use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Renders navigation indicator when message is selected
    pub fn render_navigation_indicator(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let selected_index = self.selected_message_index;
        let has_selection = selected_index.is_some();
        let total_count = self.messages.len();

        div()
            .flex()
            .items_center()
            .gap_1()
            // Navigation controls (when message selected)
            .when(has_selection, |d| {
                let idx = selected_index.unwrap_or(0);
                let position = idx + 1; // 1-based for display
                // Get role of selected message
                let role_label = self.messages.get(idx).map(|msg| {
                    match msg.role {
                        MessageRole::User => "You",
                        MessageRole::Assistant => "Claude",
                        MessageRole::ToolUse => "Tool",
                        MessageRole::ToolResult => "Result",
                        MessageRole::Error => "Error",
                        MessageRole::Thinking => "Thinking",
                        MessageRole::System => "System",
                    }
                }).unwrap_or("?");
                // Check if selected message is bookmarked
                let is_bookmarked = self.message_views.get(idx)
                    .map(|view| view.read(cx).is_bookmarked())
                    .unwrap_or(false);
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .px_2()
                        .py(px(2.0))
                        .rounded_md()
                        .bg(theme.colors.accent.opacity(0.1))
                        // First button
                        .child(
                            div()
                                .id("nav-first")
                                .px_1()
                                .rounded_sm()
                                .cursor_pointer()
                                .text_xs()
                                .text_color(if idx > 0 {
                                    theme.colors.accent
                                } else {
                                    theme.colors.text_muted.opacity(0.3)
                                })
                                .hover(|s| s.bg(theme.colors.accent.opacity(0.2)))
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.select_first_message(cx);
                                }))
                                .child("⏮")
                        )
                        // Previous button
                        .child(
                            div()
                                .id("nav-prev")
                                .px_1()
                                .rounded_sm()
                                .cursor_pointer()
                                .text_xs()
                                .text_color(if idx > 0 {
                                    theme.colors.accent
                                } else {
                                    theme.colors.text_muted.opacity(0.3)
                                })
                                .hover(|s| s.bg(theme.colors.accent.opacity(0.2)))
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.select_prev_message(cx);
                                }))
                                .child("◀")
                        )
                        // Position indicator with role
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                // Role label
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .child(role_label)
                                )
                                // Position
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.colors.accent)
                                        .child(format!("{}/{}", position, total_count))
                                )
                        )
                        // Next button
                        .child(
                            div()
                                .id("nav-next")
                                .px_1()
                                .rounded_sm()
                                .cursor_pointer()
                                .text_xs()
                                .text_color(if idx + 1 < total_count {
                                    theme.colors.accent
                                } else {
                                    theme.colors.text_muted.opacity(0.3)
                                })
                                .hover(|s| s.bg(theme.colors.accent.opacity(0.2)))
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.select_next_message(cx);
                                }))
                                .child("▶")
                        )
                        // Last button
                        .child(
                            div()
                                .id("nav-last")
                                .px_1()
                                .rounded_sm()
                                .cursor_pointer()
                                .text_xs()
                                .text_color(if idx + 1 < total_count {
                                    theme.colors.accent
                                } else {
                                    theme.colors.text_muted.opacity(0.3)
                                })
                                .hover(|s| s.bg(theme.colors.accent.opacity(0.2)))
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.select_last_message(cx);
                                }))
                                .child("⏭")
                        )
                        // Copy selected message button
                        .child(
                            div()
                                .id("nav-copy")
                                .flex()
                                .items_center()
                                .gap(px(2.0))
                                .px_1()
                                .ml_1()
                                .rounded_sm()
                                .cursor_pointer()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .hover(|s| s.bg(theme.colors.accent.opacity(0.1)).text_color(theme.colors.accent))
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.copy_selected_message(cx);
                                }))
                                .child("Copy")
                                .child(
                                    div()
                                        .text_color(theme.colors.text_muted.opacity(0.5))
                                        .child("⌥C")
                                )
                        )
                        // Bookmark toggle button
                        .child(
                            div()
                                .id("nav-bookmark")
                                .flex()
                                .items_center()
                                .gap(px(2.0))
                                .px_1()
                                .ml_1()
                                .rounded_sm()
                                .cursor_pointer()
                                .text_xs()
                                .text_color(if is_bookmarked {
                                    theme.colors.warning
                                } else {
                                    theme.colors.text_muted
                                })
                                .hover(|s| s.bg(theme.colors.warning.opacity(0.1)).text_color(theme.colors.warning))
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.bookmark_selected_message(cx);
                                }))
                                .child(if is_bookmarked { "★" } else { "☆" })
                                .child(
                                    div()
                                        .text_color(theme.colors.text_muted.opacity(0.5))
                                        .child("⌥B")
                                )
                        )
                        // Clear selection button
                        .child(
                            div()
                                .id("nav-clear")
                                .px_1()
                                .ml_1()
                                .rounded_sm()
                                .cursor_pointer()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .hover(|s| s.bg(theme.colors.error.opacity(0.1)).text_color(theme.colors.error))
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.clear_message_selection(cx);
                                }))
                                .child("✕")
                        )
                )
            })
            // Navigation hint (when no selection but has messages)
            .when(!has_selection && total_count > 1, |d| {
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .px_2()
                        .py(px(2.0))
                        .rounded_md()
                        .text_xs()
                        .text_color(theme.colors.text_muted.opacity(0.6))
                        .child("Navigate: ⌥↑↓")
                        .child("·")
                        .child("Jump: ⌥Home/End")
                )
            })
    }
}
