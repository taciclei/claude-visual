//! Git panel content rendering

use gpui::*;
use gpui::prelude::*;

use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Render git panel content with repository information
    pub fn render_git_panel_content(
        &self,
        theme: &crate::app::theme::Theme,
    ) -> impl IntoElement {
        let git_info = self.git_info.as_ref();

        div()
            .px_4()
            .py_4()
            .when(git_info.is_none(), |d| {
                d.child(
                    div()
                        .text_center()
                        .child(
                            div().text_lg().mb_2().child("📂")
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(theme.colors.text_muted)
                                .child("Not a git repository")
                        )
                )
            })
            .when_some(git_info.cloned(), |d, info| {
                d.flex()
                    .flex_col()
                    .gap_4()
                    // Branch info
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(div().text_sm().child("🌿"))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(theme.colors.text)
                                            .child(info.branch.clone())
                                    )
                            )
                            .when_some(info.sync_status(), |d, sync| {
                                d.child(
                                    div()
                                        .px_2()
                                        .py_px()
                                        .rounded_sm()
                                        .bg(theme.colors.warning.opacity(0.1))
                                        .text_xs()
                                        .text_color(theme.colors.warning)
                                        .child(sync)
                                )
                            })
                    )
                    // Status summary
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .when(info.staged_count > 0, |d| {
                                d.child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .px_2()
                                        .py_1()
                                        .rounded_sm()
                                        .bg(theme.colors.success.opacity(0.1))
                                        .text_xs()
                                        .text_color(theme.colors.success)
                                        .child("✓")
                                        .child(format!("{} staged", info.staged_count))
                                )
                            })
                            .when(info.unstaged_count > 0, |d| {
                                d.child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .px_2()
                                        .py_1()
                                        .rounded_sm()
                                        .bg(theme.colors.warning.opacity(0.1))
                                        .text_xs()
                                        .text_color(theme.colors.warning)
                                        .child("~")
                                        .child(format!("{} modified", info.unstaged_count))
                                )
                            })
                            .when(info.untracked_count > 0, |d| {
                                d.child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .px_2()
                                        .py_1()
                                        .rounded_sm()
                                        .bg(theme.colors.info.opacity(0.1))
                                        .text_xs()
                                        .text_color(theme.colors.info)
                                        .child("?")
                                        .child(format!("{} untracked", info.untracked_count))
                                )
                            })
                            .when(!info.is_dirty, |d| {
                                d.child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .text_xs()
                                        .text_color(theme.colors.success)
                                        .child("✓")
                                        .child("Working tree clean")
                                )
                            })
                    )
                    // Last commit
                    .when_some(info.last_commit.clone(), |d, commit| {
                        d.child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .child("Last commit")
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(theme.colors.text)
                                        .child(commit)
                                )
                        )
                    })
                    // Remote URL
                    .when_some(info.remote.clone(), |d, remote| {
                        let short_remote = if remote.len() > 40 {
                            format!("{}...", &remote[..37])
                        } else {
                            remote.clone()
                        };
                        d.child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text_muted)
                                        .child("Remote")
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.text.opacity(0.7))
                                        .child(short_remote)
                                )
                        )
                    })
            })
    }
}
