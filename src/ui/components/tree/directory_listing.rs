//! Directory listing component

use gpui::prelude::*;
use gpui::*;

use super::types::DirectoryEntry;

/// Directory listing component
#[derive(Clone)]
pub struct DirectoryListing {
    entries: Vec<DirectoryEntry>,
    show_hidden: bool,
}

impl DirectoryListing {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            show_hidden: false,
        }
    }

    pub fn entry(mut self, entry: DirectoryEntry) -> Self {
        self.entries.push(entry);
        self
    }

    pub fn show_hidden(mut self) -> Self {
        self.show_hidden = true;
        self
    }
}

impl Default for DirectoryListing {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for DirectoryListing {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let surface_hover = hsla(0.0, 0.0, 0.18, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);

        // Sort: directories first, then files
        let mut entries = self.entries;
        entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        });

        div().w_full().flex().flex_col().children(
            entries
                .into_iter()
                .filter(|e| self.show_hidden || !e.name.starts_with('.'))
                .map(|entry| {
                    let icon = if entry.is_dir { "📁" } else { "📄" };
                    let name_color = if entry.is_dir { accent } else { text };

                    div()
                        .h(px(32.0))
                        .w_full()
                        .px_3()
                        .flex()
                        .items_center()
                        .gap_3()
                        .cursor_pointer()
                        .hover(|s| s.bg(surface_hover))
                        .child(div().text_base().child(icon))
                        .child(
                            div()
                                .flex_1()
                                .text_sm()
                                .text_color(name_color)
                                .overflow_hidden()
                                .text_ellipsis()
                                .child(entry.name),
                        )
                        .when_some(entry.size, |d, size| {
                            d.child(
                                div()
                                    .w(px(80.0))
                                    .text_xs()
                                    .text_color(text_muted)
                                    .text_right()
                                    .child(size),
                            )
                        })
                        .when_some(entry.modified, |d, modified| {
                            d.child(
                                div()
                                    .w(px(100.0))
                                    .text_xs()
                                    .text_color(text_muted)
                                    .text_right()
                                    .child(modified),
                            )
                        })
                }),
        )
    }
}
