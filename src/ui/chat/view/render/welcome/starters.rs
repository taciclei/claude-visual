//! Starter prompts render function for ChatView

use super::super::super::core::ChatView;
use super::super::super::types::ChatViewEvent;
use crate::app::theme::Theme;
use gpui::prelude::*;
use gpui::*;

/// Starter prompt category
struct StarterCategory {
    name: &'static str,
    color_key: &'static str,
    starters: &'static [(&'static str, &'static str, &'static str, &'static str)],
}

/// All starter categories organized by workflow
const STARTER_CATEGORIES: &[StarterCategory] = &[
    StarterCategory {
        name: "Implementation",
        color_key: "accent",
        starters: &[
            (
                "⚡",
                "APEX",
                "Full workflow with validation",
                "/apex Implement ",
            ),
            ("🚀", "Oneshot", "Quick implementation", "/oneshot "),
            (
                "🧠",
                "Ultrathink",
                "Deep analysis mode",
                "/ultrathink Analyze ",
            ),
        ],
    },
    StarterCategory {
        name: "Exploration",
        color_key: "info",
        starters: &[
            ("🔍", "Explore", "Understand codebase", "/explore "),
            ("🔎", "Search", "Quick search", "/search "),
            ("📖", "Explain", "Explain code", "/explain "),
            ("💡", "Brainstorm", "Research topic", "/brainstorm "),
        ],
    },
    StarterCategory {
        name: "Code Quality",
        color_key: "success",
        starters: &[
            ("👀", "Review", "Review code", "/review-code "),
            ("♻️", "Refactor", "Improve code", "/refactor "),
            ("✨", "Clean Code", "Apply best practices", "/clean-code "),
            ("🐛", "Debug", "Debug errors", "/debug "),
        ],
    },
    StarterCategory {
        name: "Git & CI",
        color_key: "warning",
        starters: &[
            ("📦", "Commit", "Smart commit", "/commit"),
            ("🔀", "Create PR", "Open pull request", "/create-pr"),
            ("👁️", "Review PR", "Review changes", "/review"),
            ("🔧", "CI Fixer", "Fix CI failures", "/ci-fixer"),
        ],
    },
    StarterCategory {
        name: "Session",
        color_key: "text_muted",
        starters: &[
            ("📊", "Usage", "Check token usage", "/usage"),
            ("🗜️", "Compact", "Compress context", "/compact"),
            ("📝", "Memory", "Save to CLAUDE.md", "/memory"),
            ("🩺", "Doctor", "System health", "/doctor"),
        ],
    },
];

impl ChatView {
    pub fn render_starter_prompts(&self, theme: &Theme, cx: &mut Context<Self>) -> Div {
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap_4()
            .children(STARTER_CATEGORIES.iter().map(|category| {
                let category_color = match category.color_key {
                    "accent" => theme.colors.accent,
                    "info" => theme.colors.info,
                    "success" => theme.colors.success,
                    "warning" => theme.colors.warning,
                    _ => theme.colors.text_muted,
                };

                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    // Category header
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(category_color)
                            .child(category.name),
                    )
                    // Starters grid
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_2()
                            .children(category.starters.iter().map(
                                |(icon, label, preview, prompt)| {
                                    let prompt_text = prompt.to_string();
                                    let is_slash_command = prompt.starts_with('/');
                                    div()
                                        .id(SharedString::from(format!("starter-{}", label)))
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .px_3()
                                        .py_2()
                                        .rounded_lg()
                                        .bg(if is_slash_command {
                                            category_color.opacity(0.05)
                                        } else {
                                            theme.colors.surface
                                        })
                                        .border_1()
                                        .border_color(if is_slash_command {
                                            category_color.opacity(0.2)
                                        } else {
                                            theme.colors.border
                                        })
                                        .cursor_pointer()
                                        .hover(|s| {
                                            s.bg(category_color.opacity(0.15))
                                                .border_color(category_color.opacity(0.4))
                                        })
                                        .on_click(cx.listener(move |this, _, _window, cx| {
                                            // For simple commands like /commit, /usage, submit directly
                                            if prompt_text.starts_with('/')
                                                && !prompt_text.ends_with(' ')
                                            {
                                                cx.emit(ChatViewEvent::Submit(prompt_text.clone()));
                                            } else {
                                                // Set the prompt in the input for editing
                                                this.input.update(cx, |input, cx| {
                                                    input.set_text(prompt_text.clone(), cx);
                                                });
                                            }
                                            cx.notify();
                                        }))
                                        .child(div().text_base().child(*icon))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap_px()
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(if is_slash_command {
                                                            category_color
                                                        } else {
                                                            theme.colors.text
                                                        })
                                                        .child(*label),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(theme.colors.text_muted)
                                                        .max_w(px(130.0))
                                                        .overflow_hidden()
                                                        .text_ellipsis()
                                                        .child(*preview),
                                                ),
                                        )
                                },
                            )),
                    )
            }))
            // Keyboard shortcuts hint
            .child(
                div()
                    .mt_4()
                    .pt_4()
                    .border_t_1()
                    .border_color(theme.colors.border.opacity(0.3))
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap_4()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child(
                                div()
                                    .px_1()
                                    .rounded_sm()
                                    .bg(theme.colors.surface)
                                    .border_1()
                                    .border_color(theme.colors.border)
                                    .font_family("monospace")
                                    .child("/"),
                            )
                            .child("for skills"),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child(
                                div()
                                    .px_1()
                                    .rounded_sm()
                                    .bg(theme.colors.surface)
                                    .border_1()
                                    .border_color(theme.colors.border)
                                    .font_family("monospace")
                                    .child("⌘K"),
                            )
                            .child("commands"),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child(
                                div()
                                    .px_1()
                                    .rounded_sm()
                                    .bg(theme.colors.surface)
                                    .border_1()
                                    .border_color(theme.colors.border)
                                    .font_family("monospace")
                                    .child("⌘T"),
                            )
                            .child("templates"),
                    ),
            )
    }
}
