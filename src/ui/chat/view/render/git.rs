//! Git quick actions render functions for ChatView

use gpui::*;
use gpui::prelude::*;
use super::super::core::ChatView;

impl ChatView {
    pub fn render_git_quick_actions(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> Div {
        let git = match &self.git_info {
            Some(g) if g.is_dirty || g.staged_count > 0 => g,
            _ => return div(),
        };

        let branch = git.branch.clone();
        let status = git.status_summary();
        let has_staged = git.staged_count > 0;

        div()
            .flex()
            .items_center()
            .gap_2()
            .px_4()
            .py_2()
            .bg(theme.colors.surface.opacity(0.5))
            .border_b_1()
            .border_color(theme.colors.border.opacity(0.3))
            // Git branch and status
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .text_xs()
                    .child(
                        div()
                            .text_color(theme.colors.success)
                            .child(format!("⎇ {}", branch))
                    )
                    .child(
                        div()
                            .text_color(theme.colors.warning)
                            .child(status)
                    )
            )
            // Quick actions
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .ml_2()
                    // Review changes button
                    .child(
                        div()
                            .id("git-review-btn")
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
                                this.request_code_review(cx);
                            }))
                            .child("👀 Review")
                    )
                    // Commit button (only if staged)
                    .when(has_staged, |d| {
                        d.child(
                            div()
                                .id("git-commit-btn")
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
                                    this.send_slash_command("/commit", cx);
                                }))
                                .child("✓ Commit")
                        )
                    })
                    // PR button
                    .child(
                        div()
                            .id("git-pr-btn")
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .hover(|s| s.bg(theme.colors.surface_hover).text_color(theme.colors.text))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.create_pr(cx);
                            }))
                            .child("🔀 PR")
                    )
            )
    }
}
