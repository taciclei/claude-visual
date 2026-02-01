//! Git status panel button

use crate::ui::chat::view::core::ChatView;
use gpui::prelude::*;
use gpui::*;

impl ChatView {
    /// Render git status panel button (🌿 with branch and status info)
    pub(in crate::ui::chat::view::render::toolbar::quick_actions_bar) fn render_git_panel_button(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Option<Stateful<Div>> {
        let git = self.git_info.as_ref()?;

        let branch = git.branch.clone();
        let is_dirty = git.is_dirty;
        let sync_status = git.sync_status();
        let staged = git.staged_count;
        let modified = git.unstaged_count;
        let untracked = git.untracked_count;

        let text_color_active = theme.colors.accent;
        let text_color_inactive = theme.colors.text_muted;
        let text_color_dirty = theme.colors.warning;
        let text_color_hover = theme.colors.text;
        let surface_hover = theme.colors.surface_hover;

        let info_bg = theme.colors.info.opacity(0.15);
        let info_color = theme.colors.info;
        let success_bg = theme.colors.success.opacity(0.15);
        let success_color = theme.colors.success;
        let warning_bg = theme.colors.warning.opacity(0.15);
        let warning_color = theme.colors.warning;
        let text_muted_bg = theme.colors.text_muted.opacity(0.15);
        let text_muted_color = theme.colors.text_muted;

        let on_click = cx.listener(|this, _, _window, cx| {
            this.toggle_git_panel(cx);
        });

        Some(
            div()
                .id("git-panel-btn")
                .flex()
                .items_center()
                .gap_1()
                .px_2()
                .py(px(2.0))
                .rounded_md()
                .cursor_pointer()
                .text_xs()
                .text_color(if self.show_git_panel {
                    text_color_active
                } else if is_dirty {
                    text_color_dirty
                } else {
                    text_color_inactive
                })
                .hover(move |s| s.bg(surface_hover).text_color(text_color_hover))
                .on_click(on_click)
                .child("🌿")
                .child(if branch.len() > 12 {
                    format!("{}...", &branch[..9])
                } else {
                    branch
                })
                // Sync status (ahead/behind)
                .when_some(sync_status, move |d, sync| {
                    d.child(
                        div()
                            .px_1()
                            .rounded_sm()
                            .bg(info_bg)
                            .text_color(info_color)
                            .child(sync),
                    )
                })
                // Change indicators
                .when(staged > 0, move |d| {
                    d.child(
                        div()
                            .px_1()
                            .rounded_sm()
                            .bg(success_bg)
                            .text_color(success_color)
                            .child(format!("+{}", staged)),
                    )
                })
                .when(modified > 0, move |d| {
                    d.child(
                        div()
                            .px_1()
                            .rounded_sm()
                            .bg(warning_bg)
                            .text_color(warning_color)
                            .child(format!("~{}", modified)),
                    )
                })
                .when(untracked > 0, move |d| {
                    d.child(
                        div()
                            .px_1()
                            .rounded_sm()
                            .bg(text_muted_bg)
                            .text_color(text_muted_color)
                            .child(format!("?{}", untracked)),
                    )
                }),
        )
    }
}
