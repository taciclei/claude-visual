//! Quick action buttons rendering

use gpui::*;
use gpui::prelude::*;
use crate::ui::chat::view::core::ChatView;
use crate::ui::chat::view::types::ChatViewEvent;

impl ChatView {
    pub(super) fn render_quick_action_buttons(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> impl IntoElement {
        // Build context-aware actions
        let mut actions: Vec<(&str, &str, &str, bool)> = Vec::new();

        // Context-sensitive actions first
        let ctx_usage = self.context_usage_percentage();
        if ctx_usage > 0.7 {
            actions.push(("🗜️", "Compact", "/compact", true)); // Highlighted
        }

        if self.last_error.is_some() {
            actions.push(("🐛", "Debug", "/debug", true));
        }

        // Git actions when there are changes
        if let Some(ref git) = self.git_info {
            if git.staged_count > 0 {
                actions.push(("📦", "Commit", "/commit", false));
            }
            if git.is_dirty {
                actions.push(("👀", "Review", "/review", false));
            }
        }

        // Standard quick actions
        actions.push(("📊", "Usage", "/usage", false));
        if ctx_usage <= 0.7 {
            actions.push(("🗜️", "Compact", "/compact", false));
        }
        actions.push(("📝", "Memory", "/memory", false));
        actions.push(("🩺", "Doctor", "/doctor", false));
        actions.push(("❓", "Help", "/help", false));

        // Limit to 7 actions
        actions.truncate(7);

        div()
            .children(actions.iter().map(|(icon, label, command, is_urgent)| {
                let command = command.to_string();
                let urgent = *is_urgent;
                div()
                    .id(ElementId::Name(format!("quick-{}", label).into()))
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_2()
                    .py(px(2.0))
                    .rounded_md()
                    .cursor_pointer()
                    .text_xs()
                    .when(urgent, |d| {
                        d.bg(theme.colors.warning.opacity(0.15))
                            .border_1()
                            .border_color(theme.colors.warning.opacity(0.3))
                            .text_color(theme.colors.warning)
                    })
                    .when(!urgent, |d| {
                        d.text_color(theme.colors.text_muted)
                            .hover(|s| s.bg(theme.colors.surface_hover).text_color(theme.colors.text))
                    })
                    .on_click(cx.listener(move |_this, _, _window, cx| {
                        cx.emit(ChatViewEvent::Submit(command.clone()));
                    }))
                    .child(
                        div()
                            .text_xs()
                            .child(*icon)
                    )
                    .child(*label)
            }))
    }

    /// Render a dedicated skill bar for implementation workflows
    pub fn render_skill_workflow_bar(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> impl IntoElement {
        // Workflow stages: Explore -> Plan -> Implement -> Review -> Commit
        let stages = [
            ("🔍", "Explore", "/explore", "Understand the codebase"),
            ("📋", "Plan", "/plan", "Create implementation plan"),
            ("⚡", "APEX", "/apex", "Full implementation workflow"),
            ("👀", "Review", "/review-code", "Review code quality"),
            ("📦", "Commit", "/commit", "Commit changes"),
            ("🔀", "PR", "/create-pr", "Create pull request"),
        ];

        div()
            .flex()
            .items_center()
            .gap_px()
            .children(stages.iter().enumerate().map(|(idx, (icon, label, command, _desc))| {
                let command = command.to_string();
                let is_first = idx == 0;
                let is_last = idx == stages.len() - 1;

                div()
                    .id(ElementId::Name(format!("workflow-{}", label).into()))
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_2()
                    .py(px(3.0))
                    .cursor_pointer()
                    .text_xs()
                    .bg(theme.colors.accent.opacity(0.05))
                    .border_y_1()
                    .border_color(theme.colors.accent.opacity(0.2))
                    .when(is_first, |d| d.border_l_1().rounded_l_md())
                    .when(is_last, |d| d.border_r_1().rounded_r_md())
                    .text_color(theme.colors.accent)
                    .hover(|s| s.bg(theme.colors.accent.opacity(0.15)))
                    .on_click(cx.listener(move |_this, _, _window, cx| {
                        cx.emit(ChatViewEvent::Submit(command.clone()));
                    }))
                    .child(*icon)
                    .child(*label)
                    .when(!is_last, |d| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.accent.opacity(0.5))
                                .ml_1()
                                .child("→")
                        )
                    })
            }))
    }
}
