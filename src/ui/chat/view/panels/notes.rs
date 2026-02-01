//! Notes panel render functions

use gpui::prelude::*;
use gpui::*;

use super::super::core::ChatView;

impl ChatView {
    pub fn render_notes_panel(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let has_notes = self.has_notes();

        div()
            .id("notes-panel-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle_notes_panel(cx);
            }))
            .child(
                div()
                    .id("notes-panel")
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
                    // Header
                    .child(
                        div()
                            .px_4()
                            .py_3()
                            .border_b_1()
                            .border_color(theme.colors.border)
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(div().text_base().child("📝"))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.colors.text)
                                            .child("Session Notes"),
                                    ),
                            )
                            .child(
                                div()
                                    .id("close-notes-panel")
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_sm()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_notes_panel(cx);
                                    }))
                                    .child("×"),
                            ),
                    )
                    // Notes content
                    .child(
                        div()
                            .id("notes-content")
                            .flex_1()
                            .p_4()
                            .overflow_y_scroll()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(if has_notes {
                                        theme.colors.text
                                    } else {
                                        theme.colors.text_muted
                                    })
                                    .child(if has_notes {
                                        self.session_notes.clone()
                                    } else {
                                        "No notes yet. Notes are saved with your session."
                                            .to_string()
                                    }),
                            ),
                    )
                    // Tags section
                    .when(self.has_tags(), |d| {
                        d.child(
                            div()
                                .px_4()
                                .py_2()
                                .border_t_1()
                                .border_color(theme.colors.border)
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .mb_2()
                                        .child("Tags"),
                                )
                                .child(div().flex().flex_wrap().gap_1().children(
                                    self.conversation_tags.iter().map(|tag| {
                                        div()
                                            .px_2()
                                            .py_0p5()
                                            .rounded_full()
                                            .bg(theme.colors.accent.opacity(0.15))
                                            .text_xs()
                                            .text_color(theme.colors.accent)
                                            .child(format!("#{}", tag))
                                    }),
                                )),
                        )
                    })
                    // Suggested tags
                    .child({
                        let suggestions = self.suggest_tags();
                        div()
                            .px_4()
                            .py_2()
                            .border_t_1()
                            .border_color(theme.colors.border)
                            .when(!suggestions.is_empty(), |d| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .mb_2()
                                        .child("Suggested tags (click to add)"),
                                )
                                .child(
                                    div().flex().flex_wrap().gap_1().children(
                                        suggestions.iter().map(|tag| {
                                            let tag_str = *tag;
                                            div()
                                                .id(ElementId::Name(
                                                    format!("suggest-tag-{}", tag).into(),
                                                ))
                                                .px_2()
                                                .py_0p5()
                                                .rounded_full()
                                                .cursor_pointer()
                                                .bg(theme.colors.surface_hover)
                                                .text_xs()
                                                .text_color(theme.colors.text_muted)
                                                .hover(|s| {
                                                    s.bg(theme.colors.accent.opacity(0.15))
                                                        .text_color(theme.colors.accent)
                                                })
                                                .on_click(cx.listener(
                                                    move |this, _, _window, cx| {
                                                        this.add_tag(tag_str.to_string(), cx);
                                                    },
                                                ))
                                                .child(format!("+{}", tag))
                                        }),
                                    ),
                                )
                            })
                    }),
            )
    }
}
