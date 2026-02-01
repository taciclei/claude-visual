//! File dropzone component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Dropzone for file uploads
#[derive(Clone)]
pub struct FileDropzone {
    /// Label text
    label: String,
    /// Sublabel/hint
    sublabel: Option<String>,
    /// Icon
    icon: String,
    /// Accepted file types
    accepted_types: Vec<String>,
    /// Multiple files allowed
    multiple: bool,
    /// Size limit
    size_limit: FileSizeLimit,
    /// Current state
    state: UploadState,
    /// Compact mode
    compact: bool,
}

impl FileDropzone {
    pub fn new() -> Self {
        Self {
            label: "Drop files here".to_string(),
            sublabel: Some("or click to browse".to_string()),
            icon: "📁".to_string(),
            accepted_types: Vec::new(),
            multiple: true,
            size_limit: FileSizeLimit::default(),
            state: UploadState::default(),
            compact: false,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn sublabel(mut self, sublabel: impl Into<String>) -> Self {
        self.sublabel = Some(sublabel.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = icon.into();
        self
    }

    pub fn accept(mut self, types: Vec<impl Into<String>>) -> Self {
        self.accepted_types = types.into_iter().map(|t| t.into()).collect();
        self
    }

    pub fn single(mut self) -> Self {
        self.multiple = false;
        self
    }

    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    pub fn size_limit(mut self, limit: FileSizeLimit) -> Self {
        self.size_limit = limit;
        self
    }

    pub fn state(mut self, state: UploadState) -> Self {
        self.state = state;
        self
    }

    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }
}

impl Default for FileDropzone {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for FileDropzone {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.12, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);
        let success = hsla(0.38, 0.7, 0.45, 1.0);
        let error = hsla(0.0, 0.7, 0.5, 1.0);

        let (border_color, bg_color, icon_color) = match self.state {
            UploadState::Idle => (border, hsla(0.0, 0.0, 0.0, 0.0), text_muted),
            UploadState::DragOver => (accent, accent.opacity(0.1), accent),
            UploadState::Uploading => (accent, accent.opacity(0.05), accent),
            UploadState::Success => (success, success.opacity(0.1), success),
            UploadState::Error => (error, error.opacity(0.1), error),
        };

        let height = if self.compact { px(80.0) } else { px(160.0) };

        let mut dropzone = div()
            .w_full()
            .h(height)
            .rounded(px(8.0))
            .border_2()
            .border_color(border_color)
            .bg(bg_color)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_2()
            .cursor_pointer();

        // Dashed border style (simulated)
        if matches!(self.state, UploadState::Idle) {
            dropzone = dropzone.border_color(border);
        }

        // Icon
        let icon_size = if self.compact { px(24.0) } else { px(40.0) };
        dropzone = dropzone.child(
            div()
                .size(icon_size)
                .flex()
                .items_center()
                .justify_center()
                .text_2xl()
                .text_color(icon_color)
                .child(match self.state {
                    UploadState::Success => "✓".to_string(),
                    UploadState::Error => "✕".to_string(),
                    UploadState::Uploading => "⏳".to_string(),
                    _ => self.icon.clone(),
                }),
        );

        // Label
        if !self.compact || !matches!(self.state, UploadState::Idle) {
            dropzone = dropzone.child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(text)
                    .child(match self.state {
                        UploadState::DragOver => "Release to upload".to_string(),
                        UploadState::Uploading => "Uploading...".to_string(),
                        UploadState::Success => "Upload complete!".to_string(),
                        UploadState::Error => "Upload failed".to_string(),
                        _ => self.label.clone(),
                    }),
            );
        }

        // Sublabel
        if !self.compact {
            if let Some(sublabel) = &self.sublabel {
                if matches!(self.state, UploadState::Idle) {
                    dropzone = dropzone.child(
                        div()
                            .text_xs()
                            .text_color(text_muted)
                            .child(sublabel.clone()),
                    );
                }
            }

            // File type hints
            if !self.accepted_types.is_empty() && matches!(self.state, UploadState::Idle) {
                dropzone = dropzone.child(
                    div()
                        .pt_2()
                        .text_xs()
                        .text_color(text_muted)
                        .child(format!("Accepted: {}", self.accepted_types.join(", "))),
                );
            }
        }

        dropzone
    }
}
