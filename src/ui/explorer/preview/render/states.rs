//! State-specific rendering methods for different preview states

use std::path::Path;

use gpui::*;

use crate::ui::explorer::preview::core::FilePreviewPanel;
use crate::ui::explorer::preview::types::MAX_PREVIEW_SIZE;

impl FilePreviewPanel {
    /// Render empty state
    pub(super) fn render_empty(&self, cx: &Context<Self>) -> AnyElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .h_full()
            .gap_2()
            .child(
                div()
                    .text_2xl()
                    .text_color(theme.colors.text_muted)
                    .child("👁"),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(theme.colors.text_muted)
                    .child("Hover a file to preview"),
            )
            .into_any_element()
    }

    /// Render loading state
    pub(super) fn render_loading(&self, path: &Path, cx: &Context<Self>) -> AnyElement {
        let theme = self.app_state.theme.read(cx);
        let filename = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        div()
            .flex()
            .flex_col()
            .h_full()
            .child(self.render_header(&filename, None, None, cx))
            .child(
                div().flex_1().flex().items_center().justify_center().child(
                    div()
                        .text_sm()
                        .text_color(theme.colors.text_muted)
                        .child("Loading..."),
                ),
            )
            .into_any_element()
    }

    /// Render loaded file content
    pub(super) fn render_loaded(
        &self,
        path: &Path,
        content: &str,
        line_count: usize,
        file_size: u64,
        language: Option<&str>,
        cx: &Context<Self>,
    ) -> AnyElement {
        let theme = self.app_state.theme.read(cx);
        let filename = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let content = content.to_string();
        let path = path.to_path_buf();

        div()
            .flex()
            .flex_col()
            .h_full()
            .child(self.render_header(&filename, Some(file_size), language, cx))
            .child(self.render_stats(line_count, file_size, cx))
            .child(
                // Content area with line numbers
                div()
                    .flex_1()
                    .id("scroll-preview-content")
                    .overflow_y_scroll()
                    .overflow_x_scroll()
                    .bg(theme.colors.background)
                    .py_2()
                    .child(
                        div()
                            .flex()
                            // Line numbers column
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .px_2()
                                    .text_xs()
                                    .font_family("JetBrains Mono")
                                    .text_color(theme.colors.text_muted.opacity(0.5))
                                    .text_right()
                                    .border_r_1()
                                    .border_color(theme.colors.border)
                                    .children(content.lines().enumerate().map(|(i, _)| {
                                        div()
                                            .text_right()
                                            .min_w(px(24.0))
                                            .child(format!("{}", i + 1))
                                    })),
                            )
                            // Code content column
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .px_3()
                                    .text_xs()
                                    .font_family("JetBrains Mono")
                                    .text_color(theme.colors.text)
                                    .whitespace_nowrap()
                                    .children(content.lines().map(|line| {
                                        div().child(if line.is_empty() {
                                            " ".to_string()
                                        } else {
                                            line.to_string()
                                        })
                                    })),
                            ),
                    ),
            )
            .child(self.render_footer(path, cx))
            .into_any_element()
    }

    /// Render binary file state
    pub(super) fn render_binary(
        &self,
        path: &Path,
        file_size: u64,
        cx: &Context<Self>,
    ) -> AnyElement {
        let theme = self.app_state.theme.read(cx);
        let filename = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let path = path.to_path_buf();

        div()
            .flex()
            .flex_col()
            .h_full()
            .child(self.render_header(&filename, Some(file_size), None, cx))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap_2()
                    .child(
                        div()
                            .text_2xl()
                            .text_color(theme.colors.text_muted)
                            .child("📦"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text_muted)
                            .child("Binary file"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child(self.format_size(file_size)),
                    ),
            )
            .child(self.render_footer(path, cx))
            .into_any_element()
    }

    /// Render too large file state
    pub(super) fn render_too_large(
        &self,
        path: &Path,
        file_size: u64,
        cx: &Context<Self>,
    ) -> AnyElement {
        let theme = self.app_state.theme.read(cx);
        let filename = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let path = path.to_path_buf();

        div()
            .flex()
            .flex_col()
            .h_full()
            .child(self.render_header(&filename, Some(file_size), None, cx))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap_2()
                    .child(
                        div()
                            .text_2xl()
                            .text_color(theme.colors.warning)
                            .child("⚠️"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text_muted)
                            .child("File too large to preview"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child(format!(
                                "{} (max {})",
                                self.format_size(file_size),
                                self.format_size(MAX_PREVIEW_SIZE)
                            )),
                    ),
            )
            .child(self.render_footer(path, cx))
            .into_any_element()
    }

    /// Render error state
    pub(super) fn render_error(
        &self,
        path: &Path,
        message: &str,
        cx: &Context<Self>,
    ) -> AnyElement {
        let theme = self.app_state.theme.read(cx);
        let filename = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let message = message.to_string();

        div()
            .flex()
            .flex_col()
            .h_full()
            .child(self.render_header(&filename, None, None, cx))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap_2()
                    .child(div().text_2xl().text_color(theme.colors.error).child("❌"))
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.error)
                            .child(message),
                    ),
            )
            .into_any_element()
    }
}
