//! Context panel files section component

use gpui::*;
use gpui::prelude::*;

use super::super::super::core::ChatView;
use crate::app::theme::Theme;

impl ChatView {
    pub(super) fn render_files_section(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .mb_4()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .mb_2()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().text_sm().child("📁"))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.colors.text_muted)
                                    .child("Context Files")
                            )
                    )
                    .when(!self.context_files.is_empty(), |d| {
                        d.child(
                            div()
                                .id("clear-context-files")
                                .px_2()
                                .py_0p5()
                                .rounded_md()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.colors.surface_hover).text_color(theme.colors.error))
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.clear_context_files(cx);
                                }))
                                .child("Clear all")
                        )
                    })
            )
            .when(self.context_files.is_empty(), |d| {
                d.child(
                    div()
                        .px_3()
                        .py_4()
                        .rounded_md()
                        .bg(theme.colors.background)
                        .border_1()
                        .border_color(theme.colors.border.opacity(0.5))
                        .text_center()
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .child("No files in context")
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .mt_1()
                                .child("Use @file:path to add files")
                        )
                )
            })
            .when(!self.context_files.is_empty(), |d| {
                d.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .children(self.context_files.iter().map(|file| {
                            let file_path = file.path.clone();
                            div()
                                .id(ElementId::Name(format!("ctx-file-{}", file.path).into()))
                                .w_full()
                                .px_3()
                                .py_2()
                                .rounded_md()
                                .bg(theme.colors.background)
                                .border_1()
                                .border_color(theme.colors.border.opacity(0.5))
                                .flex()
                                .items_center()
                                .gap_2()
                                .hover(|s| s.border_color(theme.colors.accent.opacity(0.5)))
                                .child(
                                    div()
                                        .text_sm()
                                        .child(file.file_type.icon())
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .overflow_hidden()
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(theme.colors.text)
                                                .child(file.name.clone())
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.text_muted)
                                                .overflow_hidden()
                                                .child(file.path.clone())
                                        )
                                )
                                .child(
                                    div()
                                        .id(ElementId::Name(format!("remove-ctx-{}", file.path).into()))
                                        .size(px(20.0))
                                        .rounded_md()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .cursor_pointer()
                                        .hover(|s| s.bg(theme.colors.error.opacity(0.1)).text_color(theme.colors.error))
                                        .on_click(cx.listener(move |this, _, _window, cx| {
                                            this.remove_context_file(&file_path, cx);
                                        }))
                                        .child("×")
                                )
                        }))
                )
            })
    }
}
