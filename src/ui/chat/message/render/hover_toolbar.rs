//! Hover toolbar for quick message actions

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;
use crate::claude::message::MessageRole;
use super::super::view::MessageView;
use super::super::types::MessageAction;

impl MessageView {
    /// Render the hover toolbar that appears on message hover
    pub(in crate::ui::chat::message) fn render_hover_toolbar(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let is_user = matches!(self.message.role, MessageRole::User);
        let is_assistant = matches!(self.message.role, MessageRole::Assistant);
        let is_bookmarked = self.bookmarked;

        let surface = theme.colors.surface;
        let surface_hover = theme.colors.surface_hover;
        let text_muted = theme.colors.text_muted;
        let text = theme.colors.text;
        let warning = theme.colors.warning;
        let error = theme.colors.error;
        let border = theme.colors.border;

        div()
            .id("hover-toolbar")
            .absolute()
            .top_1()
            .right_1()
            .flex()
            .items_center()
            .gap_0p5()
            .px_1()
            .py_0p5()
            .rounded_md()
            .bg(surface)
            .border_1()
            .border_color(border)
            .shadow_sm()
            // Start invisible, become visible on parent group hover
            .opacity(0.0)
            .group_hover("message", |s| s.opacity(1.0))
            // Copy button
            .child(
                div()
                    .id("toolbar-copy")
                    .px_1p5()
                    .py_1()
                    .rounded_sm()
                    .text_sm()
                    .text_color(text_muted)
                    .cursor_pointer()
                    .hover(move |s| s.bg(surface_hover).text_color(text))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.execute_action(MessageAction::Copy, cx);
                    }))
                    .child("📋")
            )
            // Quote button
            .child(
                div()
                    .id("toolbar-quote")
                    .px_1p5()
                    .py_1()
                    .rounded_sm()
                    .text_sm()
                    .text_color(text_muted)
                    .cursor_pointer()
                    .hover(move |s| s.bg(surface_hover).text_color(text))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.execute_action(MessageAction::Quote, cx);
                    }))
                    .child("💬")
            )
            // Bookmark button
            .child(
                div()
                    .id("toolbar-bookmark")
                    .px_1p5()
                    .py_1()
                    .rounded_sm()
                    .text_sm()
                    .text_color(if is_bookmarked { warning } else { text_muted })
                    .cursor_pointer()
                    .hover(move |s| s.bg(if is_bookmarked { warning.opacity(0.2) } else { surface_hover }).text_color(warning))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.execute_action(MessageAction::Bookmark, cx);
                    }))
                    .child(if is_bookmarked { "★" } else { "☆" })
            )
            // Edit button (user messages only)
            .when(is_user, |d| {
                d.child(
                    div()
                        .id("toolbar-edit")
                        .px_1p5()
                        .py_1()
                        .rounded_sm()
                        .text_sm()
                        .text_color(text_muted)
                        .cursor_pointer()
                        .hover(move |s| s.bg(surface_hover).text_color(text))
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.execute_action(MessageAction::Edit, cx);
                        }))
                        .child("✏️")
                )
            })
            // Regenerate button (assistant messages only)
            .when(is_assistant, |d| {
                d.child(
                    div()
                        .id("toolbar-regenerate")
                        .px_1p5()
                        .py_1()
                        .rounded_sm()
                        .text_sm()
                        .text_color(text_muted)
                        .cursor_pointer()
                        .hover(move |s| s.bg(surface_hover).text_color(text))
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.execute_action(MessageAction::Regenerate, cx);
                        }))
                        .child("🔄")
                )
            })
            // Delete button
            .child(
                div()
                    .id("toolbar-delete")
                    .px_1p5()
                    .py_1()
                    .rounded_sm()
                    .text_sm()
                    .text_color(text_muted)
                    .cursor_pointer()
                    .hover(move |s| s.bg(error.opacity(0.2)).text_color(error))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.execute_action(MessageAction::Delete, cx);
                    }))
                    .child("🗑️")
            )
    }
}
