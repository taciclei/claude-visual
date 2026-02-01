//! Statistics panel render functions

use gpui::prelude::*;
use gpui::*;

use super::super::core::ChatView;

impl ChatView {
    pub fn render_stats_panel(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let stats = self.get_conversation_stats();

        div()
            .id("stats-panel-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle_stats_panel(cx);
            }))
            .child(
                div()
                    .id("stats-panel")
                    .w(px(400.0))
                    .max_h(px(500.0))
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
                                    .child(div().text_base().child("📊"))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.colors.text)
                                            .child("Conversation Statistics"),
                                    ),
                            )
                            .child(
                                div()
                                    .id("close-stats-panel")
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_sm()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_stats_panel(cx);
                                    }))
                                    .child("×"),
                            ),
                    )
                    // Messages section
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
                                    .child("Messages"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap_3()
                                    .child(self.render_stat_card(
                                        "Total",
                                        &format!("{}", stats.message_count),
                                        "💬",
                                        &theme,
                                    ))
                                    .child(self.render_stat_card(
                                        "You",
                                        &format!("{}", stats.user_message_count),
                                        "👤",
                                        &theme,
                                    ))
                                    .child(self.render_stat_card(
                                        "Claude",
                                        &format!("{}", stats.assistant_message_count),
                                        "🤖",
                                        &theme,
                                    ))
                                    .child(self.render_stat_card(
                                        "Tools",
                                        &format!("{}", stats.tool_use_count),
                                        "🔧",
                                        &theme,
                                    )),
                            ),
                    )
                    // Tokens section
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
                                    .child("Token Usage"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap_3()
                                    .child(self.render_stat_card(
                                        "Input",
                                        &Self::format_token_count(stats.input_tokens),
                                        "📥",
                                        &theme,
                                    ))
                                    .child(self.render_stat_card(
                                        "Output",
                                        &Self::format_token_count(stats.output_tokens),
                                        "📤",
                                        &theme,
                                    ))
                                    .child(self.render_stat_card(
                                        "Cost",
                                        &format!("${:.4}", stats.total_cost),
                                        "💵",
                                        &theme,
                                    )),
                            ),
                    )
                    // Organization section
                    .child(
                        div()
                            .px_4()
                            .py_3()
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.colors.text_muted)
                                    .mb_2()
                                    .child("Organization"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap_3()
                                    .child(self.render_stat_card(
                                        "Pinned",
                                        &format!("{}", stats.pinned_count),
                                        "📌",
                                        &theme,
                                    ))
                                    .child(self.render_stat_card(
                                        "Bookmarked",
                                        &format!("{}", stats.bookmarked_count),
                                        "🔖",
                                        &theme,
                                    ))
                                    .child(self.render_stat_card(
                                        "Tags",
                                        &format!("{}", stats.tags_count),
                                        "🏷️",
                                        &theme,
                                    ))
                                    .child(self.render_stat_card(
                                        "Words",
                                        &format!("{}", stats.word_count),
                                        "📏",
                                        &theme,
                                    )),
                            ),
                    )
                    // Topics section (conversation summary)
                    .child({
                        let topics = self.get_conversation_topics();
                        let tools = self.get_tools_used();

                        div()
                            .px_4()
                            .py_3()
                            .border_t_1()
                            .border_color(theme.colors.border)
                            // Topics
                            .when(!topics.is_empty(), |d| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.colors.text_muted)
                                        .mb_2()
                                        .child("Topics Discussed"),
                                )
                                .child(
                                    div().flex().flex_col().gap_1().mb_3().children(
                                        topics.into_iter().map(|topic| {
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.text)
                                                .px_2()
                                                .py_1()
                                                .rounded_sm()
                                                .bg(theme.colors.surface_hover)
                                                .child(format!("• {}", topic))
                                        }),
                                    ),
                                )
                            })
                            // Tools used
                            .when(!tools.is_empty(), |d| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.colors.text_muted)
                                        .mb_2()
                                        .child("Tools Used"),
                                )
                                .child(
                                    div().flex().flex_wrap().gap_1().children(
                                        tools.into_iter().map(|tool| {
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.info)
                                                .px_2()
                                                .py(px(2.0))
                                                .rounded_sm()
                                                .bg(theme.colors.info.opacity(0.1))
                                                .child(tool)
                                        }),
                                    ),
                                )
                            })
                    })
                    // Quick actions footer
                    .child(self.render_stats_quick_actions(theme, cx)),
            )
    }

    /// Render quick action buttons for the stats panel
    fn render_stats_quick_actions(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        use super::super::types::ChatViewEvent;

        let ctx_usage = self.context_usage_percentage();
        let show_compact = ctx_usage > 0.5;
        let has_many_messages = self.messages.len() > 5;

        div()
            .px_4()
            .py_3()
            .border_t_1()
            .border_color(theme.colors.border)
            .flex()
            .flex_col()
            .gap_3()
            // Context management section
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.colors.text_muted)
                            .child("Context Management")
                    )
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_2()
                            // Compact button (if needed)
                            .when(show_compact, |d| {
                                d.child(
                                    div()
                                        .id("stats-compact")
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .px_2()
                                        .py_1()
                                        .rounded_md()
                                        .cursor_pointer()
                                        .bg(theme.colors.warning.opacity(0.1))
                                        .border_1()
                                        .border_color(theme.colors.warning.opacity(0.2))
                                        .text_xs()
                                        .text_color(theme.colors.warning)
                                        .hover(|s| s.bg(theme.colors.warning.opacity(0.2)))
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.toggle_stats_panel(cx);
                                            cx.emit(ChatViewEvent::Submit("/compact".to_string()));
                                        }))
                                        .child("🗜️")
                                        .child("Compact")
                                )
                            })
                            // Usage details
                            .child(
                                div()
                                    .id("stats-usage")
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .bg(theme.colors.accent.opacity(0.1))
                                    .border_1()
                                    .border_color(theme.colors.accent.opacity(0.2))
                                    .text_xs()
                                    .text_color(theme.colors.accent)
                                    .hover(|s| s.bg(theme.colors.accent.opacity(0.2)))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_stats_panel(cx);
                                        cx.emit(ChatViewEvent::Submit("/usage".to_string()));
                                    }))
                                    .child("📊")
                                    .child("Full Usage")
                            )
                            // Save to memory
                            .child(
                                div()
                                    .id("stats-memory")
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .bg(theme.colors.success.opacity(0.1))
                                    .border_1()
                                    .border_color(theme.colors.success.opacity(0.2))
                                    .text_xs()
                                    .text_color(theme.colors.success)
                                    .hover(|s| s.bg(theme.colors.success.opacity(0.2)))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_stats_panel(cx);
                                        cx.emit(ChatViewEvent::Submit("/claude-memory".to_string()));
                                    }))
                                    .child("📝")
                                    .child("Memory")
                            )
                    )
            )
            // Conversation actions section (only if has messages)
            .when(has_many_messages, |d| {
                d.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.colors.text_muted)
                                .child("Conversation Actions")
                        )
                        .child(
                            div()
                                .flex()
                                .flex_wrap()
                                .gap_2()
                                // Summarize conversation
                                .child(
                                    div()
                                        .id("stats-summarize")
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .px_2()
                                        .py_1()
                                        .rounded_md()
                                        .cursor_pointer()
                                        .bg(theme.colors.info.opacity(0.1))
                                        .border_1()
                                        .border_color(theme.colors.info.opacity(0.2))
                                        .text_xs()
                                        .text_color(theme.colors.info)
                                        .hover(|s| s.bg(theme.colors.info.opacity(0.2)))
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.toggle_stats_panel(cx);
                                            cx.emit(ChatViewEvent::Submit("Summarize our conversation so far, highlighting key decisions and progress".to_string()));
                                        }))
                                        .child("📋")
                                        .child("Summarize")
                                )
                                // Create tasks from conversation
                                .child(
                                    div()
                                        .id("stats-tasks")
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .px_2()
                                        .py_1()
                                        .rounded_md()
                                        .cursor_pointer()
                                        .bg(theme.colors.surface_hover)
                                        .border_1()
                                        .border_color(theme.colors.border)
                                        .text_xs()
                                        .text_color(theme.colors.text)
                                        .hover(|s| s.bg(theme.colors.accent.opacity(0.1)))
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.toggle_stats_panel(cx);
                                            cx.emit(ChatViewEvent::Submit("Create a task list from what we've discussed. What are the next steps?".to_string()));
                                        }))
                                        .child("✅")
                                        .child("Tasks")
                                )
                                // Continue with APEX
                                .child(
                                    div()
                                        .id("stats-apex")
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .px_2()
                                        .py_1()
                                        .rounded_md()
                                        .cursor_pointer()
                                        .bg(theme.colors.surface_hover)
                                        .border_1()
                                        .border_color(theme.colors.border)
                                        .text_xs()
                                        .text_color(theme.colors.text)
                                        .hover(|s| s.bg(theme.colors.accent.opacity(0.1)))
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.toggle_stats_panel(cx);
                                            cx.emit(ChatViewEvent::Submit("/apex Continue implementing based on our discussion".to_string()));
                                        }))
                                        .child("⚡")
                                        .child("APEX")
                                )
                        )
                )
            })
            // Quick skills section
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.colors.text_muted)
                            .child("Quick Skills")
                    )
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_2()
                            // Explore
                            .child(
                                div()
                                    .id("stats-explore")
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .bg(theme.colors.surface_hover)
                                    .text_xs()
                                    .text_color(theme.colors.text)
                                    .hover(|s| s.bg(theme.colors.info.opacity(0.1)).text_color(theme.colors.info))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_stats_panel(cx);
                                        cx.emit(ChatViewEvent::Submit("/explore".to_string()));
                                    }))
                                    .child("🔍")
                                    .child("Explore")
                            )
                            // Debug
                            .child(
                                div()
                                    .id("stats-debug")
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .bg(theme.colors.surface_hover)
                                    .text_xs()
                                    .text_color(theme.colors.text)
                                    .hover(|s| s.bg(theme.colors.error.opacity(0.1)).text_color(theme.colors.error))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_stats_panel(cx);
                                        cx.emit(ChatViewEvent::Submit("/debug".to_string()));
                                    }))
                                    .child("🐛")
                                    .child("Debug")
                            )
                            // Commit
                            .child(
                                div()
                                    .id("stats-commit")
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .bg(theme.colors.surface_hover)
                                    .text_xs()
                                    .text_color(theme.colors.text)
                                    .hover(|s| s.bg(theme.colors.success.opacity(0.1)).text_color(theme.colors.success))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_stats_panel(cx);
                                        cx.emit(ChatViewEvent::Submit("/commit".to_string()));
                                    }))
                                    .child("📦")
                                    .child("Commit")
                            )
                            // Review
                            .child(
                                div()
                                    .id("stats-review")
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .bg(theme.colors.surface_hover)
                                    .text_xs()
                                    .text_color(theme.colors.text)
                                    .hover(|s| s.bg(theme.colors.warning.opacity(0.1)).text_color(theme.colors.warning))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_stats_panel(cx);
                                        cx.emit(ChatViewEvent::Submit("/review".to_string()));
                                    }))
                                    .child("👀")
                                    .child("Review")
                            )
                    )
            )
    }

    /// Render a stat card
    pub fn render_stat_card(
        &self,
        label: &str,
        value: &str,
        icon: &str,
        theme: &crate::app::theme::Theme,
    ) -> Div {
        div()
            .flex()
            .flex_col()
            .items_center()
            .px_3()
            .py_2()
            .rounded_md()
            .bg(theme.colors.surface_hover)
            .min_w(px(70.0))
            .child(div().text_sm().child(icon.to_string()))
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.colors.text)
                    .child(value.to_string()),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child(label.to_string()),
            )
    }
}
