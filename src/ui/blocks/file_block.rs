//! File preview block component

use std::path::PathBuf;
use std::sync::Arc;

use gpui::prelude::*;
use gpui::prelude::*;
use gpui::*;

use crate::app::state::AppState;

/// A file preview block
pub struct FileBlock {
    app_state: Arc<AppState>,
    path: PathBuf,
    content: Option<String>,
    #[allow(dead_code)]
    language: Option<String>,
    collapsed: bool,
    modified: bool,
}

impl FileBlock {
    pub fn new(path: PathBuf, content: Option<String>, app_state: Arc<AppState>) -> Self {
        // Detect language from extension
        let language = path.extension().and_then(|ext| {
            let ext = ext.to_string_lossy().to_lowercase();
            match ext.as_str() {
                "rs" => Some("rust".to_string()),
                "js" => Some("javascript".to_string()),
                "ts" => Some("typescript".to_string()),
                "py" => Some("python".to_string()),
                "json" => Some("json".to_string()),
                "toml" => Some("toml".to_string()),
                "md" => Some("markdown".to_string()),
                "sh" | "bash" => Some("bash".to_string()),
                _ => None,
            }
        });

        Self {
            app_state,
            path,
            content,
            language,
            collapsed: false,
            modified: false,
        }
    }

    /// Get the filename
    fn filename(&self) -> String {
        self.path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    }
}

impl RenderOnce for FileBlock {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let filename = self.filename();

        div()
            .w_full()
            .rounded_lg()
            .overflow_hidden()
            .border_1()
            .border_color(theme.colors.border)
            // Header
            .child(
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
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.colors.text_muted)
                                    .child("F"),
                            )
                            // Filename
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .child(filename),
                            )
                            // Modified indicator
                            .when(self.modified, |d| {
                                d.child(div().size(px(8.0)).rounded_full().bg(theme.colors.warning))
                            }),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            // Open button
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded_sm()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|style| {
                                        style
                                            .bg(theme.colors.surface_hover)
                                            .text_color(theme.colors.text)
                                    })
                                    .cursor_pointer()
                                    .child("Open"),
                            )
                            // Collapse button
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded_sm()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|style| {
                                        style
                                            .bg(theme.colors.surface_hover)
                                            .text_color(theme.colors.text)
                                    })
                                    .cursor_pointer()
                                    .child(if self.collapsed { "Expand" } else { "Collapse" }),
                            ),
                    ),
            )
            // Path
            .child(
                div()
                    .px_3()
                    .py_1()
                    .bg(theme.colors.surface)
                    .border_b_1()
                    .border_color(theme.colors.border)
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child(self.path.display().to_string()),
            )
            // Content
            .when(!self.collapsed, |this| {
                if let Some(content) = &self.content {
                    this.child(
                        div()
                            .w_full()
                            .max_h(px(400.0))
                            .id("scroll-file-content")
                            .overflow_y_scroll()
                            .overflow_x_scroll()
                            .bg(theme.colors.background)
                            .px_3()
                            .py_2()
                            .text_xs()
                            .font_family("JetBrains Mono")
                            .whitespace_nowrap()
                            .child(content.clone()),
                    )
                } else {
                    this.child(
                        div()
                            .w_full()
                            .px_3()
                            .py_4()
                            .bg(theme.colors.background)
                            .flex()
                            .justify_center()
                            .text_sm()
                            .text_color(theme.colors.text_muted)
                            .child("File content not available"),
                    )
                }
            })
    }
}
