//! Session details panel render functions

mod sections;

use gpui::*;
use gpui::prelude::*;

use super::super::core::ChatView;
use super::super::types::NotificationType;

impl ChatView {
    pub fn render_session_details(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let session_info = self.session_info.as_ref();

        // Extract listeners before div builder chain
        let overlay_click = cx.listener(|this, _, _window, cx| {
            this.toggle_session_details(cx);
        });

        let close_click = cx.listener(|this, _, _window, cx| {
            this.toggle_session_details(cx);
        });

        let copy_session_click = cx.listener(|this, _, _window, cx| {
            if let Some(ref info) = this.session_info {
                cx.write_to_clipboard(gpui::ClipboardItem::new_string(info.session_id.clone()));
                this.show_notification("Session ID copied", NotificationType::Success, cx);
            }
        });

        // Copy theme colors for move closures
        let surface = theme.colors.surface;
        let border = theme.colors.border;
        let surface_hover = theme.colors.surface_hover;
        let text = theme.colors.text;
        let text_muted = theme.colors.text_muted;
        let accent = theme.colors.accent;
        let accent_bg = theme.colors.accent.opacity(0.1);
        let accent_hover = theme.colors.accent.opacity(0.2);

        div()
            .id("session-details-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .on_click(overlay_click)
            .child(
                div()
                    .id("session-details-panel")
                    .w(px(550.0))
                    .max_h(px(600.0))
                    .bg(surface)
                    .rounded_lg()
                    .border_1()
                    .border_color(border)
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
                            .border_color(border)
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(div().text_lg().child("📊"))
                                    .child(div().text_base().font_weight(FontWeight::SEMIBOLD).text_color(text).child("Session Details"))
                            )
                            .child(
                                div()
                                    .id("close-session-details")
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_sm()
                                    .text_color(text_muted)
                                    .hover(move |s| s.bg(surface_hover))
                                    .on_click(close_click)
                                    .child("×")
                            )
                    )
                    // Content
                    .child(
                        div()
                            .id("session-details-content")
                            .flex_1()
                            .overflow_y_scroll()
                            .px_4()
                            .py_3()
                            .flex()
                            .flex_col()
                            .gap_4()
                            // Session Info Section
                            .when_some(session_info, |d, info| {
                                d.child(self.render_session_info_section(info, theme, cx))
                            })
                            // Token Usage Section
                            .child(self.render_token_usage_section(theme))
                            // Cost Section
                            .when(self.stats.cost > 0.0, |d| {
                                d.child(self.render_cost_section(theme))
                            })
                            // Performance Section
                            .child(self.render_performance_section(theme))
                            // Conversation Stats Section
                            .child(self.render_conversation_section(theme))
                            // Slash Commands Section
                            .when_some(session_info.filter(|i| !i.slash_commands.is_empty()), |d, info| {
                                d.child(self.render_slash_commands_section(info, theme, cx))
                            })
                            // Skills Section
                            .when_some(session_info.filter(|i| !i.skills.is_empty()), |d, info| {
                                d.child(self.render_skills_section(info, theme, cx))
                            })
                            // Agents Section
                            .when_some(session_info.filter(|i| !i.agents.is_empty()), |d, info| {
                                d.child(self.render_agents_section(info, theme, cx))
                            })
                            // Available Tools Section
                            .when_some(session_info.filter(|i| !i.tools.is_empty()), |d, info| {
                                d.child(self.render_available_tools_section(info, theme))
                            })
                    )
                    // Footer
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .border_t_1()
                            .border_color(border)
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(text_muted)
                                    .child("⌥S to toggle")
                            )
                            .child(
                                div()
                                    .id("copy-session-id-btn")
                                    .px_3()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_xs()
                                    .bg(accent_bg)
                                    .text_color(accent)
                                    .hover(move |s| s.bg(accent_hover))
                                    .on_click(copy_session_click)
                                    .child("Copy Session ID")
                            )
                    )
            )
    }
}
