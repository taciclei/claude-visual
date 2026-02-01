//! File upload list component

use super::preview::FilePreview;
use gpui::prelude::*;
use gpui::*;

/// File upload list with multiple files
#[derive(Clone)]
pub struct FileUploadList {
    files: Vec<FilePreview>,
    show_total: bool,
}

impl FileUploadList {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            show_total: true,
        }
    }

    pub fn files(mut self, files: Vec<FilePreview>) -> Self {
        self.files = files;
        self
    }

    pub fn show_total(mut self, show: bool) -> Self {
        self.show_total = show;
        self
    }
}

impl Default for FileUploadList {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for FileUploadList {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);

        let total_size: u64 = self.files.iter().map(|f| f.size_bytes).sum();
        let file_count = self.files.len();

        div()
            .w_full()
            .flex()
            .flex_col()
            .gap_2()
            // File list
            .children(self.files)
            // Total summary
            .when(self.show_total && file_count > 0, |d| {
                d.child(
                    div()
                        .w_full()
                        .pt_2()
                        .flex()
                        .justify_between()
                        .text_xs()
                        .text_color(text_muted)
                        .child(format!("{} file(s)", file_count))
                        .child(format!("Total: {}", FilePreview::format_size(total_size))),
                )
            })
    }
}
