//! Simple file tree for stateless rendering

use gpui::*;
use gpui::prelude::*;

use super::types::FileTreeItem;

/// Simple file tree for stateless rendering
#[derive(Clone)]
pub struct FileTree {
    items: Vec<FileTreeItem>,
    indent: f32,
}

impl FileTree {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            indent: 16.0,
        }
    }

    pub fn item(mut self, item: FileTreeItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn indent(mut self, indent: f32) -> Self {
        self.indent = indent;
        self
    }
}

impl Default for FileTree {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for FileTree {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.18, 1.0);

        div()
            .w_full()
            .flex()
            .flex_col()
            .children(self.items.into_iter().map(|item| {
                let icon = if item.is_dir {
                    if item.expanded { "📂" } else { "📁" }
                } else {
                    "📄"
                };
                let chevron = if item.is_dir {
                    if item.expanded { "▼" } else { "▶" }
                } else {
                    " "
                };

                div()
                    .h(px(26.0))
                    .w_full()
                    .pl(px(item.depth as f32 * self.indent))
                    .pr_2()
                    .flex()
                    .items_center()
                    .gap_1()
                    .rounded(px(4.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(surface_hover))
                    .child(
                        div()
                            .w(px(14.0))
                            .text_xs()
                            .text_color(text_muted)
                            .child(chevron)
                    )
                    .child(
                        div()
                            .text_sm()
                            .child(icon)
                    )
                    .child(
                        div()
                            .flex_1()
                            .text_sm()
                            .text_color(text)
                            .overflow_hidden()
                            .text_ellipsis()
                            .child(item.name)
                    )
            }))
    }
}
