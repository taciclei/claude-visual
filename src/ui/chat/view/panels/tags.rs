//! Tags editor panel render functions

use gpui::*;
use gpui::prelude::*;

use super::super::core::ChatView;

impl ChatView {
    pub fn render_tags_panel(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let current_tags = self.get_tags();
        let suggested_tags = self.suggest_tags();

        div()
            .id("tags-panel-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle_tags_editor(cx);
            }))
            .child(
                div()
                    .id("tags-panel")
                    .w(px(400.0))
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
                                    .child(div().text_base().child("🏷️"))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.colors.text)
                                            .child("Conversation Tags")
                                    )
                            )
                            .child(
                                div()
                                    .id("close-tags-panel")
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_sm()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_tags_editor(cx);
                                    }))
                                    .child("×")
                            )
                    )
                    // Current tags
                    .child(
                        div()
                            .px_4()
                            .py_3()
                            .border_b_1()
                            .border_color(theme.colors.border)
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.colors.text_muted)
                                    .mb_2()
                                    .child("Current Tags")
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap_1()
                                    .when(current_tags.is_empty(), |d| {
                                        d.child(
                                            div()
                                                .text_sm()
                                                .text_color(theme.colors.text_muted)
                                                .child("No tags added")
                                        )
                                    })
                                    .children(current_tags.iter().enumerate().map(|(i, tag)| {
                                        let tag_clone = tag.clone();
                                        div()
                                            .id(ElementId::Name(format!("tag-{}", i).into()))
                                            .flex()
                                            .items_center()
                                            .gap_1()
                                            .px_2()
                                            .py_0p5()
                                            .rounded_md()
                                            .bg(theme.colors.accent.opacity(0.15))
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.colors.accent)
                                                    .child(tag.clone())
                                            )
                                            .child(
                                                div()
                                                    .id(ElementId::Name(format!("remove-tag-{}", i).into()))
                                                    .cursor_pointer()
                                                    .text_xs()
                                                    .text_color(theme.colors.text_muted)
                                                    .hover(|s| s.text_color(theme.colors.error))
                                                    .on_click(cx.listener(move |this, _, _window, cx| {
                                                        this.remove_tag(&tag_clone, cx);
                                                    }))
                                                    .child("×")
                                            )
                                    }))
                            )
                    )
                    // Suggested tags
                    .child(
                        div()
                            .px_4()
                            .py_3()
                            .flex_1()
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.colors.text_muted)
                                    .mb_2()
                                    .child("Suggested Tags")
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap_1()
                                    .when(suggested_tags.is_empty(), |d| {
                                        d.child(
                                            div()
                                                .text_sm()
                                                .text_color(theme.colors.text_muted)
                                                .child("Add messages to get suggestions")
                                        )
                                    })
                                    .children(suggested_tags.iter().filter(|t| !current_tags.contains(&t.to_string())).map(|tag| {
                                        let tag_str = tag.to_string();
                                        div()
                                            .id(ElementId::Name(format!("suggest-tag-{}", tag).into()))
                                            .px_2()
                                            .py_0p5()
                                            .rounded_md()
                                            .bg(theme.colors.surface_hover)
                                            .cursor_pointer()
                                            .hover(|s| s.bg(theme.colors.accent.opacity(0.15)))
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap_1()
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(theme.colors.text_muted)
                                                            .child("+")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(theme.colors.text)
                                                            .child(*tag)
                                                    )
                                            )
                                            .on_click(cx.listener(move |this, _, _window, cx| {
                                                this.add_tag(tag_str.clone(), cx);
                                            }))
                                    }))
                            )
                    )
                    // Quick add common tags
                    .child(
                        div()
                            .px_4()
                            .py_3()
                            .border_t_1()
                            .border_color(theme.colors.border)
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.colors.text_muted)
                                    .mb_2()
                                    .child("Quick Add")
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap_1()
                                    .children(["important", "todo", "review", "archived"].iter().filter(|t| !current_tags.contains(&t.to_string())).map(|tag| {
                                        let tag_str = tag.to_string();
                                        div()
                                            .id(ElementId::Name(format!("quick-tag-{}", tag).into()))
                                            .px_2()
                                            .py_0p5()
                                            .rounded_md()
                                            .border_1()
                                            .border_color(theme.colors.border)
                                            .cursor_pointer()
                                            .hover(|s| s.bg(theme.colors.surface_hover))
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.colors.text_muted)
                                                    .child(*tag)
                                            )
                                            .on_click(cx.listener(move |this, _, _window, cx| {
                                                this.add_tag(tag_str.clone(), cx);
                                            }))
                                    }))
                            )
                    )
            )
    }

}
