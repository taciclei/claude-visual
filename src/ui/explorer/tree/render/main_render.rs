//! Main Render trait implementation

use gpui::*;
use gpui::prelude::*;

use crate::ui::explorer::tree::core::FileTree;
use crate::ui::explorer::tree::types::{default_colors, FileTreeEvent};

impl Render for FileTree {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = default_colors();
        let is_focused = self.focus_handle.is_focused(_window);

        // Extract theme colors for closures
        let accent_color = theme.accent;
        let text_color = theme.text;
        let text_muted_color = theme.text_muted;

        // Extract listeners before div chains
        let on_key_down = cx.listener(|this, event, window, cx| {
            this.handle_key(event, window, cx);
        });

        let on_new_file = cx.listener(|this, _, _window, cx| {
            if let Some(path) = this.selected_path.clone()
                .or_else(|| this.root_path.clone())
            {
                cx.emit(FileTreeEvent::NewFileRequested(path));
            }
        });

        let on_new_folder = cx.listener(|this, _, _window, cx| {
            if let Some(path) = this.selected_path.clone()
                .or_else(|| this.root_path.clone())
            {
                cx.emit(FileTreeEvent::NewFolderRequested(path));
            }
        });

        let on_refresh = cx.listener(|this, _, _window, cx| {
            this.refresh(cx);
        });

        div()
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.surface)
            .border_1()
            .border_color(if is_focused {
                theme.accent
            } else {
                theme.border
            })
            .rounded_md()
            .overflow_hidden()
            .on_key_down(on_key_down)
            .child(
                // Header
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .py_2()
                    .bg(theme.background)
                    .border_b_1()
                    .border_color(theme.border)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.text)
                                    .child("Explorer"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            // New file button
                            .child(
                                div()
                                    .id("new-file-button")
                                    .px_1()
                                    .py_px()
                                    .text_xs()
                                    .text_color(text_muted_color)
                                    .cursor_pointer()
                                    .hover(move |s| s.text_color(text_color))
                                    .on_click(on_new_file)
                                    .child("📄"),
                            )
                            // New folder button
                            .child(
                                div()
                                    .id("new-folder-button")
                                    .px_1()
                                    .py_px()
                                    .text_xs()
                                    .text_color(text_muted_color)
                                    .cursor_pointer()
                                    .hover(move |s| s.text_color(text_color))
                                    .on_click(on_new_folder)
                                    .child("📁"),
                            )
                            // Refresh button
                            .child(
                                div()
                                    .id("refresh-button")
                                    .px_1()
                                    .py_px()
                                    .text_xs()
                                    .text_color(text_muted_color)
                                    .cursor_pointer()
                                    .hover(move |s| s.text_color(text_color))
                                    .on_click(on_refresh)
                                    .child("🔄"),
                            ),
                    ),
            )
            .child(
                // Search/filter input
                div()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(theme.border)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .px_2()
                            .py_1()
                            .bg(theme.background)
                            .border_1()
                            .border_color(theme.border)
                            .rounded_sm()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child("🔍"),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .text_sm()
                                    .text_color(if self.filter.is_empty() {
                                        theme.text_muted
                                    } else {
                                        theme.text
                                    })
                                    .child(if self.filter.is_empty() {
                                        "Filter files...".to_string()
                                    } else {
                                        self.filter.clone()
                                    }),
                            ),
                    ),
            )
            .child(
                // File tree content
                div()
                    .flex_1()
                    .id("scroll-file-tree")
                    .overflow_y_scroll()
                    .child(self.render_content(&theme, cx)),
            )
    }
}
