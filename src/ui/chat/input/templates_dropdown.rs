//! Prompt templates dropdown

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;

use super::ChatInput;

/// Prompt template definition
struct PromptTemplate {
    icon: &'static str,
    label: &'static str,
    template: &'static str,
}

/// Template category for grouping
struct TemplateCategory {
    name: &'static str,
    templates: &'static [PromptTemplate],
}

/// All template categories
const TEMPLATE_CATEGORIES: &[TemplateCategory] = &[
    TemplateCategory {
        name: "Claude Code Skills",
        templates: &[
            PromptTemplate { icon: "⚡", label: "APEX Workflow", template: "/apex Implement: " },
            PromptTemplate { icon: "🧠", label: "Ultra Think", template: "/ultrathink Analyze: " },
            PromptTemplate { icon: "🚀", label: "Oneshot", template: "/oneshot Quick implement: " },
            PromptTemplate { icon: "🔍", label: "Explore", template: "/explore " },
            PromptTemplate { icon: "💡", label: "Brainstorm", template: "/brainstorm Research: " },
            PromptTemplate { icon: "🔎", label: "Search", template: "/search " },
            PromptTemplate { icon: "📖", label: "Explain", template: "/explain " },
            PromptTemplate { icon: "🐛", label: "Debug", template: "/debug " },
        ],
    },
    TemplateCategory {
        name: "Git Operations",
        templates: &[
            PromptTemplate { icon: "📦", label: "Commit", template: "/commit" },
            PromptTemplate { icon: "🔀", label: "Create PR", template: "/create-pr" },
            PromptTemplate { icon: "👀", label: "Review", template: "/review" },
            PromptTemplate { icon: "🔗", label: "Merge", template: "/merge" },
            PromptTemplate { icon: "💬", label: "Fix PR Comments", template: "/fix-pr-comments" },
        ],
    },
    TemplateCategory {
        name: "Code Quality",
        templates: &[
            PromptTemplate { icon: "♻️", label: "Refactor", template: "/refactor " },
            PromptTemplate { icon: "✨", label: "Clean Code", template: "/clean-code " },
            PromptTemplate { icon: "👁️", label: "Review Code", template: "/review-code " },
            PromptTemplate { icon: "🔧", label: "CI Fixer", template: "/ci-fixer" },
            PromptTemplate { icon: "📚", label: "Docs Research", template: "/docs " },
        ],
    },
    TemplateCategory {
        name: "Session Management",
        templates: &[
            PromptTemplate { icon: "📊", label: "Usage", template: "/usage" },
            PromptTemplate { icon: "📦", label: "Compact", template: "/compact" },
            PromptTemplate { icon: "🔄", label: "Resume", template: "/resume" },
            PromptTemplate { icon: "🧠", label: "Memory", template: "/memory" },
            PromptTemplate { icon: "🩺", label: "Doctor", template: "/doctor" },
        ],
    },
    TemplateCategory {
        name: "Quick Prompts",
        templates: &[
            PromptTemplate { icon: "💡", label: "Explain code", template: "Explain this code:\n\n```\n\n```" },
            PromptTemplate { icon: "🧪", label: "Write tests", template: "Write tests for:\n\n```\n\n```" },
            PromptTemplate { icon: "🐛", label: "Fix bug", template: "Fix this bug:\n\nError: " },
            PromptTemplate { icon: "🏗️", label: "Implement", template: "Implement:\n\n**Feature:**\n**Requirements:**\n- " },
        ],
    },
];

/// Common prompt templates (flat list for backwards compatibility)
const TEMPLATES: &[PromptTemplate] = &[
    PromptTemplate { icon: "⚡", label: "APEX", template: "/apex " },
    PromptTemplate { icon: "🔍", label: "Explore", template: "/explore " },
    PromptTemplate { icon: "🐛", label: "Debug", template: "/debug " },
    PromptTemplate { icon: "📦", label: "Commit", template: "/commit" },
];

impl ChatInput {
    /// Render templates dropdown with categories
    pub(super) fn render_templates_dropdown(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("templates-dropdown")
            .absolute()
            .bottom(px(80.0))
            .left(px(16.0))
            .w(px(300.0))
            .max_h(px(400.0))
            .overflow_y_scroll()
            .bg(theme.colors.surface)
            .border_1()
            .border_color(theme.colors.border)
            .rounded_lg()
            .shadow_lg()
            .p_1()
            .flex()
            .flex_col()
            .gap_px()
            // Header
            .child(
                div()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(theme.colors.border)
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.colors.text)
                            .child("Templates & Skills")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child("Click to insert")
                    )
            )
            // Categories with templates
            .children(
                TEMPLATE_CATEGORIES.iter().map(|category| {
                    div()
                        .flex()
                        .flex_col()
                        // Category header
                        .child(
                            div()
                                .px_3()
                                .py_1()
                                .mt_1()
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(theme.colors.accent)
                                .child(category.name)
                        )
                        // Category templates
                        .children(
                            category.templates.iter().map(|template| {
                                let tpl_text = template.template.to_string();
                                div()
                                    .id(ElementId::Name(format!("tpl-{}", template.label).into()))
                                    .px_3()
                                    .py_1p5()
                                    .mx_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                    .on_click(cx.listener(move |this, _, _window, cx| {
                                        this.insert_template(&tpl_text, cx);
                                    }))
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .w(px(18.0))
                                            .text_sm()
                                            .text_center()
                                            .child(template.icon)
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .text_sm()
                                            .text_color(theme.colors.text)
                                            .child(template.label)
                                    )
                            })
                        )
                })
            )
            // Footer with keyboard hint
            .child(
                div()
                    .mt_1()
                    .pt_1()
                    .border_t_1()
                    .border_color(theme.colors.border)
                    .px_3()
                    .py_1()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child("Type / for commands")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child("Esc to close")
                    )
            )
    }
}
