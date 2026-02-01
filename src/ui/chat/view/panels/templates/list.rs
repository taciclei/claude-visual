//! Templates panel list with categories

use gpui::*;
use gpui::prelude::*;
use std::collections::HashMap;

use super::super::super::core::ChatView;
use super::super::super::types::PromptTemplate;

impl ChatView {
    pub fn render_templates_list(
        &self,
        theme: &crate::app::theme::Theme,
        templates_by_cat: HashMap<&str, Vec<&PromptTemplate>>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let categories = ["coding", "review", "explain", "git", "architecture", "custom"];
        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;
        let border_color = theme.colors.border;
        let background = theme.colors.background;
        let surface_hover = theme.colors.surface_hover;
        let accent_color = theme.colors.accent;
        let error_color = theme.colors.error;

        div()
            .id("templates-list")
            .flex_1()
            .overflow_y_scroll()
            .p_4()
            .children(categories.iter().filter_map(|&cat| {
                templates_by_cat.get(cat).map(|templates| {
                    let cat_icon = match cat {
                        "coding" => "💻",
                        "review" => "🔍",
                        "explain" => "💡",
                        "git" => "📦",
                        "architecture" => "🏗️",
                        "custom" => "📌",
                        _ => "📄",
                    };
                    let cat_label = match cat {
                        "coding" => "Coding",
                        "review" => "Code Review",
                        "explain" => "Explanations",
                        "git" => "Git & PRs",
                        "architecture" => "Architecture",
                        "custom" => "Custom Templates",
                        _ => cat,
                    };

                    let border_fade = border_color.opacity(0.5);

                    div()
                        .mb_4()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .mb_2()
                                .child(
                                    div()
                                        .text_xs()
                                        .child(cat_icon.to_string())
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(text_muted)
                                        .child(cat_label.to_string())
                                )
                                .child(
                                    div()
                                        .h(px(1.0))
                                        .flex_1()
                                        .bg(border_fade)
                                )
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .children(templates.iter().map(|template| {
                                    self.render_template_item(
                                        theme,
                                        template,
                                        text_color,
                                        text_muted,
                                        border_color,
                                        background,
                                        surface_hover,
                                        accent_color,
                                        error_color,
                                        cx,
                                    )
                                }))
                        )
                })
            }))
            .when(templates_by_cat.is_empty(), move |d| {
                d.child(
                    div()
                        .py_8()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap_3()
                        .child(
                            div()
                                .size(px(48.0))
                                .rounded_full()
                                .bg(text_muted.opacity(0.1))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(div().text_xl().child("📝"))
                        )
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(text_color)
                                .child("No templates found")
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(text_muted)
                                .child("Try a different search term")
                        )
                )
            })
    }

    fn render_template_item(
        &self,
        _theme: &crate::app::theme::Theme,
        template: &PromptTemplate,
        text_color: Hsla,
        text_muted: Hsla,
        border_color: Hsla,
        background: Hsla,
        surface_hover: Hsla,
        accent_color: Hsla,
        error_color: Hsla,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let template_id = template.id.clone();
        let template_id_delete = template.id.clone();
        let is_builtin = template.is_builtin;
        let usage_count = template.usage_count;
        let template_name = template.name.clone();
        let template_content = template.content.chars().take(100).collect::<String>();
        let template_icon = template.icon.to_string();

        let use_listener = cx.listener(move |this, _, _window, cx| {
            this.use_template(&template_id, cx);
        });
        let delete_listener = cx.listener(move |this, _, _window, cx| {
            this.delete_template(&template_id_delete, cx);
        });

        let border_fade = border_color.opacity(0.5);
        let accent_fade = accent_color.opacity(0.5);
        let accent_bg = accent_color.opacity(0.1);
        let error_bg = error_color.opacity(0.1);

        div()
            .id(ElementId::Name(format!("template-{}", template.id).into()))
            .w_full()
            .px_3()
            .py_2()
            .rounded_md()
            .border_1()
            .border_color(border_fade)
            .bg(background)
            .cursor_pointer()
            .hover(move |s| s.bg(surface_hover).border_color(accent_fade))
            .on_click(use_listener)
            .flex()
            .items_start()
            .gap_3()
            .child(
                div()
                    .size(px(24.0))
                    .rounded_md()
                    .bg(accent_bg)
                    .flex()
                    .items_center()
                    .justify_center()
                    .flex_shrink_0()
                    .text_sm()
                    .child(template_icon)
            )
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(text_color)
                            .child(template_name)
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(text_muted)
                            .line_clamp(2)
                            .child(template_content)
                    )
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .flex_shrink_0()
                    .when(usage_count > 0, move |d| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(text_muted)
                                .child(format!("{}x", usage_count))
                        )
                    })
                    .when(!is_builtin, move |d| {
                        d.child(
                            div()
                                .id(ElementId::Name(format!("delete-template-{}", template.id).into()))
                                .size(px(20.0))
                                .rounded_md()
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_xs()
                                .text_color(text_muted)
                                .hover(move |s| s.bg(error_bg).text_color(error_color))
                                .cursor_pointer()
                                .on_click(delete_listener)
                                .child("×")
                        )
                    })
            )
    }
}
