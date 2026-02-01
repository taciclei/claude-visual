//! File preview component

use gpui::*;
use gpui::prelude::*;

/// File preview item
#[derive(Clone, IntoElement)]
pub struct FilePreview {
    /// File name
    pub(crate) filename: String,
    /// File size in bytes
    pub(crate) size_bytes: u64,
    /// File type/extension
    file_type: Option<String>,
    /// Upload progress (0-100)
    progress: Option<f32>,
    /// Error message
    error: Option<String>,
    /// Show remove button
    removable: bool,
}

impl FilePreview {
    pub fn new(filename: impl Into<String>, size_bytes: u64) -> Self {
        Self {
            filename: filename.into(),
            size_bytes,
            file_type: None,
            progress: None,
            error: None,
            removable: true,
        }
    }

    pub fn file_type(mut self, file_type: impl Into<String>) -> Self {
        self.file_type = Some(file_type.into());
        self
    }

    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = Some(progress.clamp(0.0, 100.0));
        self
    }

    pub fn error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn not_removable(mut self) -> Self {
        self.removable = false;
        self
    }

    pub(crate) fn format_size(bytes: u64) -> String {
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.1} KB", bytes as f64 / 1024.0)
        } else if bytes < 1024 * 1024 * 1024 {
            format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }

    fn file_icon(file_type: &Option<String>) -> &'static str {
        match file_type.as_deref() {
            Some("pdf") => "📄",
            Some("doc") | Some("docx") => "📝",
            Some("xls") | Some("xlsx") => "📊",
            Some("png") | Some("jpg") | Some("jpeg") | Some("gif") => "🖼️",
            Some("zip") | Some("rar") | Some("7z") => "📦",
            Some("mp3") | Some("wav") | Some("ogg") => "🎵",
            Some("mp4") | Some("mov") | Some("avi") => "🎬",
            Some("rs") | Some("py") | Some("js") | Some("ts") => "💻",
            _ => "📁",
        }
    }
}

impl RenderOnce for FilePreview {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.15, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.18, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);
        let error = hsla(0.0, 0.7, 0.5, 1.0);

        let has_error = self.error.is_some();
        let is_uploading = self.progress.is_some() && self.progress.unwrap() < 100.0;

        div()
            .w_full()
            .p_3()
            .rounded(px(6.0))
            .bg(surface)
            .border_1()
            .border_color(if has_error { error } else { border })
            .flex()
            .items_center()
            .gap_3()
            // File icon
            .child(
                div()
                    .size(px(36.0))
                    .rounded(px(6.0))
                    .bg(hsla(0.0, 0.0, 0.1, 1.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_lg()
                    .child(Self::file_icon(&self.file_type))
            )
            // File info
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .overflow_hidden()
                    // Filename
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(text)
                            .overflow_hidden()
                            .text_ellipsis()
                            .child(self.filename.clone())
                    )
                    // Size or error
                    .child(
                        div()
                            .text_xs()
                            .text_color(if has_error { error } else { text_muted })
                            .child(if let Some(err) = &self.error {
                                err.clone()
                            } else {
                                Self::format_size(self.size_bytes)
                            })
                    )
                    // Progress bar
                    .when_some(self.progress, |d, progress| {
                        d.child(
                            div()
                                .w_full()
                                .h(px(4.0))
                                .mt_1()
                                .rounded(px(2.0))
                                .bg(hsla(0.0, 0.0, 0.1, 1.0))
                                .child(
                                    div()
                                        .h_full()
                                        .w(relative(progress / 100.0))
                                        .rounded(px(2.0))
                                        .bg(accent)
                                )
                        )
                    })
            )
            // Status/Remove button
            .child(
                div()
                    .flex_shrink_0()
                    .child(
                        if is_uploading {
                            div()
                                .text_xs()
                                .text_color(text_muted)
                                .child(format!("{}%", self.progress.unwrap_or(0.0) as u32))
                                .into_any_element()
                        } else if has_error {
                            div()
                                .size(px(24.0))
                                .rounded(px(4.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_color(error)
                                .cursor_pointer()
                                .hover(|s| s.bg(surface_hover))
                                .child("↻")
                                .into_any_element()
                        } else if self.removable {
                            div()
                                .size(px(24.0))
                                .rounded(px(4.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_color(text_muted)
                                .cursor_pointer()
                                .hover(|s| s.bg(surface_hover).text_color(text))
                                .child("×")
                                .into_any_element()
                        } else {
                            div()
                                .text_color(hsla(0.38, 0.7, 0.45, 1.0))
                                .child("✓")
                                .into_any_element()
                        }
                    )
            )
    }
}
