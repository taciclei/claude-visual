//! File picker render functions

use gpui::prelude::*;
use gpui::*;

use super::super::core::ChatView;

impl ChatView {
    pub fn render_file_picker(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let files = &self.file_picker.results;
        let query = &self.file_picker.query;

        div()
            .id("file-picker-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle_file_picker(cx);
            }))
            .child(
                div()
                    .id("file-picker")
                    .w(px(500.0))
                    .max_h(px(400.0))
                    .bg(theme.colors.surface)
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.colors.border)
                    .shadow_lg()
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    .on_click(|_, _, _| {})
                    // Header with search
                    .child(
                        div()
                            .px_4()
                            .py_3()
                            .border_b_1()
                            .border_color(theme.colors.border)
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().text_sm().child("📎"))
                            .child(
                                div()
                                    .flex_1()
                                    .text_sm()
                                    .text_color(if query.is_empty() {
                                        theme.colors.text_muted
                                    } else {
                                        theme.colors.text
                                    })
                                    .child(if query.is_empty() {
                                        "Search files to mention...".to_string()
                                    } else {
                                        query.clone()
                                    }),
                            )
                            .child(
                                div()
                                    .id("close-file-picker")
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_sm()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_file_picker(cx);
                                    }))
                                    .child("×"),
                            ),
                    )
                    // File list
                    .child(
                        div()
                            .id("file-picker-list")
                            .flex_1()
                            .overflow_y_scroll()
                            .when(files.is_empty(), |d| {
                                d.child(
                                    div()
                                        .px_4()
                                        .py_8()
                                        .text_center()
                                        .text_sm()
                                        .text_color(theme.colors.text_muted)
                                        .child("Type to search files..."),
                                )
                            })
                            .children(files.iter().enumerate().map(|(idx, file)| {
                                let path = file.path.clone();
                                div()
                                    .id(SharedString::from(format!("file-{}", idx)))
                                    .px_4()
                                    .py_2()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                    .on_click(cx.listener(move |this, _, _window, cx| {
                                        this.select_file(&path, cx);
                                    }))
                                    .child(div().text_sm().child(file.icon()))
                                    .child(
                                        div()
                                            .flex_1()
                                            .flex()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(theme.colors.text)
                                                    .child(file.name.clone()),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.colors.text_muted)
                                                    .child(file.path.clone()),
                                            ),
                                    )
                            })),
                    )
                    // Footer hint
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .border_t_1()
                            .border_color(theme.colors.border)
                            .flex()
                            .items_center()
                            .justify_between()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child("Click to insert @mention")
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        div()
                                            .px_1()
                                            .rounded_sm()
                                            .bg(theme.colors.background)
                                            .border_1()
                                            .border_color(theme.colors.border)
                                            .font_family("monospace")
                                            .child("⎋"),
                                    )
                                    .child("Close"),
                            ),
                    ),
            )
    }
}
