//! Session history panel render functions

use gpui::prelude::*;
use gpui::*;

use super::super::core::ChatView;
use crate::claude::message::MessageRole;
use crate::ui::pct;

impl ChatView {
    pub fn render_session_history_panel(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let sessions: Vec<_> = self.recent_sessions.iter().enumerate().collect();

        // Full-screen overlay
        div()
            .id("session-history-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            // Click outside to close
            .on_click(cx.listener(|this, _, _window, cx| {
                this.toggle_session_history(cx);
            }))
            .child(
                // Panel
                div()
                    .id("session-history-panel")
                    .w(px(500.0))
                    .max_h(px(500.0))
                    .bg(theme.colors.surface)
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.colors.border)
                    .shadow_lg()
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    // Prevent clicks from propagating
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
                                    .child(div().text_base().child("📋"))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.colors.text)
                                            .child("Recent Sessions"),
                                    ),
                            )
                            .child(
                                div()
                                    .id("close-session-history")
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_sm()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.toggle_session_history(cx);
                                    }))
                                    .child("×"),
                            ),
                    )
                    // Sessions list
                    .child(
                        div()
                            .id("session-list")
                            .flex_1()
                            .overflow_y_scroll()
                            .when(sessions.is_empty(), |d| {
                                d.child(self.render_session_history_empty_state(theme, cx))
                            })
                            .children(sessions.into_iter().map(|(idx, session)| {
                                let session_id = session.session_id.clone();
                                let elapsed =
                                    chrono::Utc::now().signed_duration_since(session.last_active);
                                let time_str = if elapsed.num_hours() < 1 {
                                    format!("{} min ago", elapsed.num_minutes())
                                } else if elapsed.num_hours() < 24 {
                                    format!("{} hr ago", elapsed.num_hours())
                                } else {
                                    format!("{} days ago", elapsed.num_days())
                                };

                                div()
                                    .id(SharedString::from(format!("session-{}", idx)))
                                    .px_4()
                                    .py_3()
                                    .border_b_1()
                                    .border_color(theme.colors.border.opacity(0.5))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                    .on_click(cx.listener(move |this, _, _window, cx| {
                                        this.resume_session(&session_id, cx);
                                    }))
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    // Title row
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(theme.colors.text)
                                                    .child(session.title.clone()),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.colors.text_muted)
                                                    .child(time_str),
                                            ),
                                    )
                                    // Info row
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_3()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap_1()
                                                    .child("💬")
                                                    .child(format!(
                                                        "{} messages",
                                                        session.message_count
                                                    )),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap_1()
                                                    .child("🤖")
                                                    .child(session.model.clone()),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap_1()
                                                    .child("📁")
                                                    .child(session.cwd.clone()),
                                            ),
                                    )
                            })),
                    )
                    // Footer
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
                            .child("Click a session to resume")
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

    fn render_session_history_empty_state(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        use super::super::types::ChatViewEvent;

        let accent = theme.colors.accent;
        let info = theme.colors.info;

        div()
            .px_4()
            .py_8()
            .flex()
            .flex_col()
            .items_center()
            .gap_3()
            .child(div().text_3xl().child("📋"))
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.colors.text)
                    .child("No recent sessions"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .text_center()
                    .child("Start a new conversation or resume a previous one"),
            )
            // Quick skill suggestions
            .child(
                div()
                    .pt_3()
                    .flex()
                    .flex_wrap()
                    .justify_center()
                    .gap_2()
                    // Start with APEX
                    .child(
                        div()
                            .id("session-empty-apex")
                            .px_3()
                            .py_2()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(accent.opacity(0.15))
                            .border_1()
                            .border_color(accent.opacity(0.3))
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(accent)
                            .hover(move |s| {
                                s.bg(accent.opacity(0.25)).border_color(accent.opacity(0.5))
                            })
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.toggle_session_history(cx);
                                cx.emit(ChatViewEvent::Submit("/apex".to_string()));
                            }))
                            .child("⚡ Start with APEX"),
                    )
                    // Explore codebase
                    .child(
                        div()
                            .id("session-empty-explore")
                            .px_3()
                            .py_2()
                            .rounded_md()
                            .cursor_pointer()
                            .bg(info.opacity(0.15))
                            .border_1()
                            .border_color(info.opacity(0.3))
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(info)
                            .hover(move |s| {
                                s.bg(info.opacity(0.25)).border_color(info.opacity(0.5))
                            })
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.toggle_session_history(cx);
                                cx.emit(ChatViewEvent::Submit("/explore".to_string()));
                            }))
                            .child("🔍 Explore Codebase"),
                    ),
            )
    }
}
