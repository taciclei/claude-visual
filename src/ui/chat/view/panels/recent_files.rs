//! Recent files panel render functions

use gpui::prelude::*;
use gpui::*;

use super::super::core::ChatView;
use super::super::types::ChatViewEvent;

impl ChatView {
    pub fn render_recent_files_panel(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let recent_files = &self.recent_files;
        let has_files = !recent_files.is_empty();

        div()
            .id("recent-files-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle_recent_files_panel(cx);
            }))
            .child(
                div()
                    .id("recent-files-panel")
                    .w(px(550.0))
                    .max_h(px(550.0))
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
                                    .child(div().text_base().child("📂"))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.colors.text)
                                            .child("Recent Files"),
                                    )
                                    .child(
                                        div()
                                            .px_2()
                                            .py_px()
                                            .rounded_full()
                                            .bg(theme.colors.accent.opacity(0.1))
                                            .text_xs()
                                            .text_color(theme.colors.accent)
                                            .child(format!("{}", recent_files.len())),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .when(has_files, |d| {
                                        d.child(
                                            div()
                                                .id("clear-recent-files")
                                                .px_2()
                                                .py_1()
                                                .rounded_md()
                                                .cursor_pointer()
                                                .text_xs()
                                                .text_color(theme.colors.error)
                                                .hover(|s| s.bg(theme.colors.error.opacity(0.1)))
                                                .on_click(cx.listener(|this, _, _window, cx| {
                                                    this.clear_recent_files(cx);
                                                }))
                                                .child("Clear All"),
                                        )
                                    })
                                    .child(
                                        div()
                                            .id("close-recent-files")
                                            .px_2()
                                            .py_1()
                                            .rounded_md()
                                            .cursor_pointer()
                                            .text_sm()
                                            .text_color(theme.colors.text_muted)
                                            .hover(|s| s.bg(theme.colors.surface_hover))
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                this.toggle_recent_files_panel(cx);
                                            }))
                                            .child("×"),
                                    ),
                            ),
                    )
                    // Files list
                    .child(
                        div()
                            .id("recent-files-list")
                            .flex_1()
                            .overflow_y_scroll()
                            .when(!has_files, |d| {
                                d.child(self.render_empty_files_state(theme, cx))
                            })
                            .when(has_files, |d| {
                                d.children(recent_files.iter().enumerate().map(|(idx, file)| {
                                    let file_path = file.path.clone();
                                    let file_path_ctx = file.path.clone();
                                    let file_path_explain = file.path.clone();
                                    let file_path_review = file.path.clone();
                                    let file_icon = file.file_type.icon();
                                    let time_ago = Self::format_time_ago(file.accessed_at);

                                    div()
                                        .id(ElementId::Name(format!("recent-file-{}", idx).into()))
                                        .flex()
                                        .flex_col()
                                        .px_4()
                                        .py_2()
                                        .border_b_1()
                                        .border_color(theme.colors.border.opacity(0.5))
                                        .hover(|s| s.bg(theme.colors.surface_hover.opacity(0.5)))
                                        // File info row
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .justify_between()
                                                .child(
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap_3()
                                                        .flex_1()
                                                        .overflow_hidden()
                                                        .child(div().text_lg().child(file_icon))
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .flex_col()
                                                                .overflow_hidden()
                                                                .child(
                                                                    div()
                                                                        .text_sm()
                                                                        .text_color(
                                                                            theme.colors.text,
                                                                        )
                                                                        .overflow_hidden()
                                                                        .text_ellipsis()
                                                                        .child(file.name.clone()),
                                                                )
                                                                .child(
                                                                    div()
                                                                        .text_xs()
                                                                        .text_color(
                                                                            theme.colors.text_muted,
                                                                        )
                                                                        .overflow_hidden()
                                                                        .text_ellipsis()
                                                                        .child(file.path.clone()),
                                                                ),
                                                        ),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(theme.colors.text_muted)
                                                        .child(time_ago),
                                                ),
                                        )
                                        // Actions row with skills
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_2()
                                                .mt_2()
                                                .pl(px(32.0))
                                                // Basic actions
                                                .child(
                                                    div()
                                                        .id(ElementId::Name(
                                                            format!("add-ctx-{}", idx).into(),
                                                        ))
                                                        .px_2()
                                                        .py_1()
                                                        .rounded_md()
                                                        .cursor_pointer()
                                                        .text_xs()
                                                        .text_color(theme.colors.info)
                                                        .hover(|s| {
                                                            s.bg(theme.colors.info.opacity(0.1))
                                                        })
                                                        .on_click(cx.listener(
                                                            move |this, _, _window, cx| {
                                                                this.add_context_file(
                                                                    file_path_ctx.clone(),
                                                                    cx,
                                                                );
                                                            },
                                                        ))
                                                        .child("+ctx"),
                                                )
                                                .child(
                                                    div()
                                                        .id(ElementId::Name(
                                                            format!("mention-{}", idx).into(),
                                                        ))
                                                        .px_2()
                                                        .py_1()
                                                        .rounded_md()
                                                        .cursor_pointer()
                                                        .text_xs()
                                                        .text_color(theme.colors.accent)
                                                        .hover(|s| {
                                                            s.bg(theme.colors.accent.opacity(0.1))
                                                        })
                                                        .on_click(cx.listener(
                                                            move |this, _, _window, cx| {
                                                                this.insert_file_mention(
                                                                    &file_path, cx,
                                                                );
                                                                this.toggle_recent_files_panel(cx);
                                                            },
                                                        ))
                                                        .child("@mention"),
                                                )
                                                // Separator
                                                .child(
                                                    div()
                                                        .w_px()
                                                        .h(px(14.0))
                                                        .bg(theme.colors.border)
                                                        .mx_1(),
                                                )
                                                // Skill actions
                                                .child(
                                                    div()
                                                        .id(ElementId::Name(
                                                            format!("explain-{}", idx).into(),
                                                        ))
                                                        .flex()
                                                        .items_center()
                                                        .gap_1()
                                                        .px_2()
                                                        .py_1()
                                                        .rounded_md()
                                                        .cursor_pointer()
                                                        .text_xs()
                                                        .text_color(theme.colors.text_muted)
                                                        .hover(|s| {
                                                            s.bg(theme.colors.surface_hover)
                                                                .text_color(theme.colors.text)
                                                        })
                                                        .on_click(cx.listener(
                                                            move |this, _, _window, cx| {
                                                                this.toggle_recent_files_panel(cx);
                                                                let prompt = format!(
                                                                    "/explain @file:{}",
                                                                    file_path_explain
                                                                );
                                                                cx.emit(ChatViewEvent::Submit(
                                                                    prompt,
                                                                ));
                                                            },
                                                        ))
                                                        .child("📖")
                                                        .child("Explain"),
                                                )
                                                .child(
                                                    div()
                                                        .id(ElementId::Name(
                                                            format!("review-{}", idx).into(),
                                                        ))
                                                        .flex()
                                                        .items_center()
                                                        .gap_1()
                                                        .px_2()
                                                        .py_1()
                                                        .rounded_md()
                                                        .cursor_pointer()
                                                        .text_xs()
                                                        .text_color(theme.colors.text_muted)
                                                        .hover(|s| {
                                                            s.bg(theme.colors.surface_hover)
                                                                .text_color(theme.colors.text)
                                                        })
                                                        .on_click(cx.listener(
                                                            move |this, _, _window, cx| {
                                                                this.toggle_recent_files_panel(cx);
                                                                let prompt = format!(
                                                                    "/review @file:{}",
                                                                    file_path_review
                                                                );
                                                                cx.emit(ChatViewEvent::Submit(
                                                                    prompt,
                                                                ));
                                                            },
                                                        ))
                                                        .child("👀")
                                                        .child("Review"),
                                                ),
                                        )
                                }))
                            }),
                    )
                    // Footer with file skills
                    .child(self.render_file_skills_footer(theme, cx)),
            )
    }

    /// Render empty files state with skill suggestions
    fn render_empty_files_state(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .p_8()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_2xl()
                            .text_color(theme.colors.text_muted.opacity(0.3))
                            .child("📂"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text_muted)
                            .child("No recent files"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted.opacity(0.6))
                            .child("Mention files with @file:path"),
                    ),
            )
            // Explore codebase button
            .child(
                div()
                    .id("explore-codebase-btn")
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_4()
                    .py_2()
                    .rounded_md()
                    .cursor_pointer()
                    .bg(theme.colors.accent.opacity(0.1))
                    .border_1()
                    .border_color(theme.colors.accent.opacity(0.2))
                    .text_sm()
                    .text_color(theme.colors.accent)
                    .hover(|s| s.bg(theme.colors.accent.opacity(0.2)))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.toggle_recent_files_panel(cx);
                        cx.emit(ChatViewEvent::Submit(
                            "/explore What are the main files in this codebase?".to_string(),
                        ));
                    }))
                    .child("🔍")
                    .child("Explore Codebase"),
            )
    }

    /// Render file skills footer
    fn render_file_skills_footer(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .px_4()
            .py_2()
            .border_t_1()
            .border_color(theme.colors.border)
            .flex()
            .items_center()
            .justify_between()
            // Hint
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child("Use @file:path to mention files"),
            )
            // Quick file skills
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Explore
                    .child(
                        div()
                            .id("file-skill-explore")
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .text_xs()
                            .text_color(theme.colors.info)
                            .hover(|s| s.bg(theme.colors.info.opacity(0.1)))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.toggle_recent_files_panel(cx);
                                cx.emit(ChatViewEvent::Submit("/explore".to_string()));
                            }))
                            .child("🔍")
                            .child("Explore"),
                    )
                    // Search
                    .child(
                        div()
                            .id("file-skill-search")
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .text_xs()
                            .text_color(theme.colors.accent)
                            .hover(|s| s.bg(theme.colors.accent.opacity(0.1)))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.toggle_recent_files_panel(cx);
                                cx.emit(ChatViewEvent::Submit("/search".to_string()));
                            }))
                            .child("⚡")
                            .child("Search"),
                    ),
            )
    }

    /// Format time ago
    fn format_time_ago(time: chrono::DateTime<chrono::Utc>) -> String {
        let now = chrono::Utc::now();
        let diff = now.signed_duration_since(time);

        if diff.num_seconds() < 60 {
            "just now".to_string()
        } else if diff.num_minutes() < 60 {
            format!("{}m ago", diff.num_minutes())
        } else if diff.num_hours() < 24 {
            format!("{}h ago", diff.num_hours())
        } else {
            format!("{}d ago", diff.num_days())
        }
    }
}
