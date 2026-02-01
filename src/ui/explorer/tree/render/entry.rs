//! Individual file/folder entry rendering

use gpui::*;
use gpui::prelude::*;

use crate::ui::explorer::tree::core::FileTree;
use crate::ui::explorer::tree::types::{SimpleColors, DragPreview, DraggedFile};
use crate::ui::explorer::file_item::FileEntry;

impl FileTree {
    pub(crate) fn render_entry(&self, entry: &FileEntry, theme: &SimpleColors, cx: &mut Context<Self>) -> impl IntoElement {
        let path = entry.path.clone();
        let path_for_click = path.clone();
        let path_for_dbl = path.clone();
        let path_for_context = path.clone();
        let path_for_drag = path.clone();
        let path_expand = path.clone();
        let is_selected = self.selected_path.as_ref() == Some(&path);
        let is_directory = entry.is_directory();
        let depth = entry.depth;
        let entry_name = entry.name.clone();
        let entry_icon = entry.icon();

        // Copy theme colors for move closures
        let surface_hover_color = theme.surface_hover;
        let text_muted_color = theme.text_muted;
        let accent_color = theme.accent;

        // Extract listeners before div chain
        let on_click = cx.listener(move |this, _, _window, cx| {
            this.select(path_for_click.clone(), cx);
        });

        let on_expand = cx.listener(move |this, _, _window, cx| {
            this.toggle_expand(&path_expand, cx);
        });

        let on_add_context = cx.listener(move |this, _, _window, cx| {
            this.add_to_context(path_for_context.clone(), cx);
        });

        div()
            .id(ElementId::Name(format!("tree-entry-{}", path.display()).into()))
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py_1()
            .pl(px(8.0 + depth as f32 * 16.0))
            .cursor_pointer()
            .bg(if is_selected {
                theme.accent.opacity(0.2)
            } else {
                gpui::transparent_black()
            })
            .hover(move |s| s.bg(surface_hover_color))
            .on_click(on_click)
            // Drag support for attaching to context
            .on_drag(
                DraggedFile::new(path_for_drag.clone()),
                move |dragged, _offset, _window, cx| {
                    // Drag preview showing file name with icon
                    cx.new(|_| {
                        DragPreview {
                            is_directory: dragged.is_directory,
                            name: dragged.name.clone(),
                        }
                    })
                },
            )
            // Expand/collapse icon for directories
            .when(entry.is_directory(), |el| {
                el.child(
                    div()
                        .id(ElementId::Name(format!("expand-{}", path.display()).into()))
                        .w_4()
                        .text_xs()
                        .text_color(text_muted_color)
                        .on_click(on_expand)
                        .child(if entry.is_expanded { "▼" } else { "▶" }),
                )
            })
            .when(!entry.is_directory(), |el| {
                el.child(div().w_4()) // Spacer
            })
            // File icon
            .child(
                div()
                    .text_sm()
                    .child(entry.icon()),
            )
            // File name
            .child(
                div()
                    .flex_1()
                    .text_sm()
                    .text_color(theme.text)
                    .text_ellipsis()
                    .child(entry.name.clone()),
            )
            // Git status indicator
            .when(entry.git_status.is_some(), |el| {
                let status = entry.git_status.unwrap();
                let (r, g, b) = status.color();
                el.child(
                    div()
                        .text_xs()
                        .text_color(rgb((r as u32) << 16 | (g as u32) << 8 | b as u32))
                        .child(status.char().to_string()),
                )
            })
            // Add to context button (on hover)
            .child(
                div()
                    .id(ElementId::Name(format!("add-context-{}", path.display()).into()))
                    .text_xs()
                    .text_color(text_muted_color)
                    .cursor_pointer()
                    .hover(move |s| s.text_color(accent_color))
                    .on_click(on_add_context)
                    .child("+"),
            )
    }
}
