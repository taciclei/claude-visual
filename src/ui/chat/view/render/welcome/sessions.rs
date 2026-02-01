//! Recent sessions cards render function for ChatView

use super::super::super::core::ChatView;
use super::super::super::types::{ChatViewEvent, NotificationType};
use crate::app::theme::Theme;
use gpui::prelude::*;
use gpui::*;

impl ChatView {
    pub fn render_recent_sessions_cards(&self, theme: &Theme, cx: &mut Context<Self>) -> Div {
        // Show up to 4 recent sessions
        let sessions: Vec<_> = self.recent_sessions.iter().take(4).collect();

        div()
            .w_full()
            .flex()
            .flex_col()
            .gap_2()
            .children(sessions.iter().map(|session| {
                let session_id = session.session_id.clone();
                let title = session.title.clone();
                let last_active = session.last_active;
                let msg_count = session.message_count;
                let model = session.model.clone();
                let cwd = session.cwd.clone();

                // Format relative time
                let now = chrono::Utc::now();
                let duration = now.signed_duration_since(last_active);
                let time_str = if duration.num_minutes() < 1 {
                    "Just now".to_string()
                } else if duration.num_minutes() < 60 {
                    format!("{}m ago", duration.num_minutes())
                } else if duration.num_hours() < 24 {
                    format!("{}h ago", duration.num_hours())
                } else if duration.num_days() < 7 {
                    format!("{}d ago", duration.num_days())
                } else {
                    last_active.format("%b %d").to_string()
                };

                // Get just the folder name from cwd
                let folder_name = cwd.split('/').last().unwrap_or(&cwd).to_string();

                div()
                    .id(SharedString::from(format!("recent-session-{}", session_id)))
                    .w_full()
                    .flex()
                    .items_center()
                    .gap_3()
                    .px_4()
                    .py_3()
                    .rounded_lg()
                    .bg(theme.colors.surface)
                    .border_1()
                    .border_color(theme.colors.border)
                    .cursor_pointer()
                    .hover(|s| {
                        s.bg(theme.colors.surface_hover)
                            .border_color(theme.colors.accent.opacity(0.5))
                    })
                    .on_click(cx.listener(move |this, _, _window, cx| {
                        let session_id = session_id.clone();
                        this.show_notification(
                            &format!("Resuming session..."),
                            NotificationType::Info,
                            cx,
                        );
                        cx.emit(ChatViewEvent::Submit(format!("/resume {}", session_id)));
                    }))
                    // Session icon
                    .child(
                        div()
                            .w(px(36.0))
                            .h(px(36.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_md()
                            .bg(theme.colors.accent.opacity(0.1))
                            .text_color(theme.colors.accent)
                            .child("💬"),
                    )
                    // Session info
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .overflow_hidden()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.colors.text)
                                    .overflow_hidden()
                                    .text_ellipsis()
                                    .child(title),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_1()
                                            .child("📁")
                                            .child(folder_name),
                                    )
                                    .child("·")
                                    .child(format!("{} msgs", msg_count))
                                    .child("·")
                                    .child(model),
                            ),
                    )
                    // Time
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child(time_str),
                    )
            }))
    }
}
