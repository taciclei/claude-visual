//! Title editor render functions

use gpui::*;
use gpui::prelude::*;
use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Renders the title section (display mode or edit mode)
    pub fn render_title_section(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let display_title = self.display_title();
        let has_custom_title = self.conversation_title.is_some();
        let is_editing_title = self.editing_title;
        let title_edit_buffer = self.title_edit_buffer.clone();

        div()
            .flex()
            .items_center()
            .gap_1()
            // Conversation title (display mode)
            .when(!is_editing_title, |d| {
                d.child(self.render_title_display(display_title, has_custom_title, theme, cx))
            })
            // Conversation title (edit mode)
            .when(is_editing_title, |d| {
                d.child(self.render_title_editor(title_edit_buffer, theme, cx))
            })
    }

    /// Renders the title display (clickable to enter edit mode)
    pub fn render_title_display(
        &self,
        display_title: String,
        has_custom_title: bool,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .id("conversation-title")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py_1()
            .rounded_md()
            .cursor_pointer()
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .text_color(if has_custom_title {
                theme.colors.text
            } else {
                theme.colors.text_muted
            })
            .hover(|s| s.bg(theme.colors.surface_hover))
            .on_click(cx.listener(|this, _, window, cx| {
                this.start_editing_title(window, cx);
            }))
            .child(display_title)
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted.opacity(0.5))
                    .ml_1()
                    .child("✎")
            )
    }

    /// Renders the title editor (input field with save/cancel buttons)
    pub fn render_title_editor(
        &self,
        title_edit_buffer: String,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let title_focus = self.title_focus.clone();
        div()
            .flex()
            .items_center()
            .gap_2()
            // Title input
            .child(
                div()
                    .id("title-edit-input")
                    .track_focus(&title_focus)
                    .px_2()
                    .py_1()
                    .min_w(px(200.0))
                    .rounded_md()
                    .bg(theme.colors.background)
                    .border_1()
                    .border_color(theme.colors.accent)
                    .text_sm()
                    .text_color(if title_edit_buffer.is_empty() {
                        theme.colors.text_muted
                    } else {
                        theme.colors.text
                    })
                    .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                        this.handle_title_key_down(event, window, cx);
                    }))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .child(if title_edit_buffer.is_empty() {
                                "Enter title...".to_string()
                            } else {
                                format!("{}_", title_edit_buffer) // Show cursor
                            })
                    )
            )
            // Hint text
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted.opacity(0.6))
                    .child("Enter to save · Esc to cancel")
            )
            // Save button
            .child(
                div()
                    .id("save-title")
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .text_xs()
                    .bg(theme.colors.accent.opacity(0.1))
                    .text_color(theme.colors.accent)
                    .hover(|s| s.bg(theme.colors.accent.opacity(0.2)))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.save_edited_title(cx);
                    }))
                    .child("Save")
            )
            // Cancel button
            .child(
                div()
                    .id("cancel-title-edit")
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .hover(|s| s.bg(theme.colors.surface_hover))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.cancel_editing_title(cx);
                    }))
                    .child("Cancel")
            )
    }
}
