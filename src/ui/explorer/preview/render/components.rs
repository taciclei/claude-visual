//! UI components for file preview (header, stats, footer)

use std::path::PathBuf;

use gpui::*;
use gpui::prelude::*;

use crate::ui::explorer::preview::core::FilePreviewPanel;
use crate::ui::explorer::preview::types::FilePreviewEvent;

impl FilePreviewPanel {
    /// Render the header with filename and close button
    pub(super) fn render_header(
        &self,
        filename: &str,
        file_size: Option<u64>,
        language: Option<&str>,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        let on_close = cx.listener(|_this, _, _window, cx| {
            cx.emit(FilePreviewEvent::Close);
        });

        div()
            .flex()
            .items_center()
            .justify_between()
            .px_3()
            .py_2()
            .bg(theme.colors.surface)
            .border_b_1()
            .border_color(theme.colors.border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // File icon
                    .child(div().text_sm().child("📄"))
                    // Filename
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.colors.text)
                            .child(filename.to_string()),
                    )
                    // Language badge
                    .when_some(language, |d, lang| {
                        d.child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded_sm()
                                .bg(theme.colors.accent.opacity(0.1))
                                .text_xs()
                                .text_color(theme.colors.accent)
                                .child(lang.to_string()),
                        )
                    }),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // File size
                    .when_some(file_size, |d, size| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .child(self.format_size(size)),
                        )
                    })
                    // Close button
                    .child(
                        div()
                            .id("preview-close")
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .hover(|s| {
                                s.bg(theme.colors.surface_hover)
                                    .text_color(theme.colors.text)
                            })
                            .cursor_pointer()
                            .on_click(on_close)
                            .child("×"),
                    ),
            )
    }

    /// Render file stats
    pub(super) fn render_stats(&self, line_count: usize, file_size: u64, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .flex()
            .items_center()
            .gap_4()
            .px_3()
            .py_1()
            .bg(theme.colors.surface)
            .border_b_1()
            .border_color(theme.colors.border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child("Lines:"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text)
                            .child(format!("{}", line_count)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child("Size:"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text)
                            .child(self.format_size(file_size)),
                    ),
            )
    }

    /// Render footer with action buttons
    pub(super) fn render_footer(&self, path: PathBuf, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let path_for_open = path.clone();
        let path_for_context = path;

        let on_add_to_context = cx.listener(move |_this, _, _window, cx| {
            cx.emit(FilePreviewEvent::AddToContext(path_for_context.clone()));
        });

        let on_open = cx.listener(move |_this, _, _window, cx| {
            cx.emit(FilePreviewEvent::OpenFile(path_for_open.clone()));
        });

        let accent_color = theme.colors.accent;
        let accent_hover = theme.colors.accent.opacity(0.9);

        div()
            .flex()
            .items_center()
            .justify_end()
            .gap_2()
            .px_3()
            .py_2()
            .border_t_1()
            .border_color(theme.colors.border)
            // Add to context button
            .child(
                div()
                    .id("add-to-context")
                    .px_2()
                    .py_1()
                    .rounded_sm()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .hover(|s| {
                        s.bg(theme.colors.surface_hover)
                            .text_color(theme.colors.text)
                    })
                    .cursor_pointer()
                    .on_click(on_add_to_context)
                    .child("+ Add to Context"),
            )
            // Open button
            .child(
                div()
                    .id("open-file")
                    .px_2()
                    .py_1()
                    .rounded_sm()
                    .text_xs()
                    .bg(accent_color)
                    .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                    .hover(move |s| s.bg(accent_hover))
                    .cursor_pointer()
                    .on_click(on_open)
                    .child("Open"),
            )
    }
}
