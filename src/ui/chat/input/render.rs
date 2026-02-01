//! Rendering implementation for ChatInput

use gpui::prelude::*;
use gpui::*;

use crate::ui::explorer::{DraggedFile, DraggedFiles};

use super::ChatInput;

impl Render for ChatInput {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Clone theme immediately to avoid borrow issues
        let theme = self.app_state.theme.read(cx).clone();
        let is_focused = self.focus_handle.is_focused(window);
        let text_is_empty = self.text.is_empty();
        let can_submit = !self.text.trim().is_empty() && !self.is_disabled;
        let is_disabled = self.is_disabled;
        let file_count = self.file_mentions().len();
        let has_mentions = !self.mentions.is_empty();
        let is_drag_over = self.is_drag_over;

        // Get vim mode info
        let vim_mode_label = self.get_vim_mode_label(&theme, cx);

        // Pre-render context chips to avoid borrow issues
        let context_chips = if file_count > 0 {
            Some(self.render_context_chips(&theme, cx))
        } else {
            None
        };

        div()
            .w_full()
            .relative()
            .flex()
            .flex_col()
            .gap_2()
            // Drag-and-drop support for file attachment
            .drag_over::<DraggedFile>(|style, _, _window, _cx| {
                style
                    .bg(hsla(210.0 / 360.0, 0.5, 0.5, 0.1))
                    .border_2()
                    .border_color(hsla(210.0 / 360.0, 0.7, 0.5, 0.6))
                    .rounded_lg()
            })
            .drag_over::<DraggedFiles>(|style, _, _window, _cx| {
                style
                    .bg(hsla(210.0 / 360.0, 0.5, 0.5, 0.1))
                    .border_2()
                    .border_color(hsla(210.0 / 360.0, 0.7, 0.5, 0.6))
                    .rounded_lg()
            })
            .on_drop(cx.listener(|this, file: &DraggedFile, window, cx| {
                this.is_drag_over = false;
                this.handle_file_drop(file, window, cx);
            }))
            .on_drop(cx.listener(|this, files: &DraggedFiles, window, cx| {
                this.is_drag_over = false;
                this.handle_files_drop(files, window, cx);
            }))
            // Drag overlay indicator
            .when(is_drag_over, |d| d.child(self.render_drag_overlay()))
            // Vim mode indicator
            .when_some(vim_mode_label, |d, (label, color)| {
                d.child(self.render_vim_indicator(label, color, &theme))
            })
            // Mention badges (show attached files)
            .when_some(context_chips, |d, chips| d.child(chips))
            // Slash command autocomplete dropdown
            .when(
                self.show_command_autocomplete && !self.filtered_commands.is_empty(),
                |d| d.child(self.render_command_dropdown(&theme, cx)),
            )
            // File mention autocomplete dropdown
            .when(
                self.show_file_autocomplete && !self.filtered_files.is_empty(),
                |d| d.child(self.render_file_dropdown(&theme, cx)),
            )
            // Prompt templates dropdown
            .when(self.show_templates, |d| {
                d.child(self.render_templates_dropdown(&theme, cx))
            })
            .child(
                div()
                    .px_4()
                    .py_3()
                    .flex()
                    .flex_row()
                    .gap_3()
                    .items_end()
                    // Disabled overlay effect
                    .when(is_disabled, |d| d.opacity(0.6))
                    // Quick action buttons (attach, command)
                    .child(self.render_toolbar(&theme, cx))
                    // Text input area
                    .child(self.render_input_area(
                        &theme,
                        is_focused,
                        is_disabled,
                        text_is_empty,
                        has_mentions,
                        cx,
                    ))
                    // Send button with keyboard hint
                    .child(self.render_send_button(&theme, can_submit, cx)),
            )
            // Footer with character count and hints
            .child(self.render_footer(&theme, text_is_empty, is_disabled))
    }
}
