//! Context files bar render function for ChatView

use gpui::*;
use gpui::prelude::*;
use crate::app::theme::Theme;
use super::super::super::core::ChatView;
use super::super::super::types::ContextFileType;

impl ChatView {
    pub fn render_context_files_bar(&self, theme: &Theme, cx: &mut Context<Self>) -> Div {
        let total_tokens: u64 = self.context_files.iter().map(|f| f.tokens).sum();
        let file_count = self.context_files.len();

        div()
            .flex()
            .items_center()
            .gap_2()
            .px_4()
            .py_2()
            .bg(theme.colors.surface.opacity(0.5))
            .border_t_1()
            .border_color(theme.colors.border.opacity(0.3))
            // Files list
            .child(
                div()
                    .id("context-files-scroll")
                    .flex()
                    .items_center()
                    .gap_2()
                    .flex_1()
                    .overflow_x_scroll()
                    .children(self.context_files.iter().enumerate().map(|(idx, file)| {
                        let file_path = file.path.clone();
                        let file_name = file.name.clone();
                        let icon = match file.file_type {
                            ContextFileType::Code => "📄",
                            ContextFileType::Markdown => "📝",
                            ContextFileType::Image => "🖼️",
                            ContextFileType::Data => "📊",
                            ContextFileType::Config => "⚙️",
                            ContextFileType::Other => "📎",
                        };

                        div()
                            .id(SharedString::from(format!("context-file-{}", idx)))
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(theme.colors.surface)
                            .border_1()
                            .border_color(theme.colors.border)
                            .cursor_pointer()
                            .group("file-item")
                            .hover(|s| s.border_color(theme.colors.accent.opacity(0.5)))
                            .child(
                                div()
                                    .text_xs()
                                    .child(icon)
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text)
                                    .max_w(px(120.0))
                                    .overflow_hidden()
                                    .text_ellipsis()
                                    .child(file_name.clone())
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child(format!("~{}k", file.tokens / 1000))
                            )
                            // Remove button (hidden by default, shown on hover)
                            .child(
                                div()
                                    .id(SharedString::from(format!("remove-file-{}", idx)))
                                    .ml_1()
                                    .w(px(16.0))
                                    .h(px(16.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded(px(2.0))
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.colors.error.opacity(0.1)).text_color(theme.colors.error))
                                    .on_click(cx.listener(move |this, _, _window, cx| {
                                        this.remove_context_file(&file_path, cx);
                                    }))
                                    .child("×")
                            )
                    }))
            )
            // Summary
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child(format!("{} files", file_count))
                    .child("·")
                    .child(format!("~{}k tokens", total_tokens / 1000))
            )
            // Clear all button
            .child(
                div()
                    .id("clear-all-context")
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .cursor_pointer()
                    .hover(|s| s.bg(theme.colors.error.opacity(0.1)).text_color(theme.colors.error))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.clear_context_files(cx);
                    }))
                    .child("Clear All")
            )
    }
}
