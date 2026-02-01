//! Git panel actions rendering

use gpui::*;
use gpui::prelude::*;

use crate::ui::chat::view::core::ChatView;
use crate::ui::chat::view::types::ChatViewEvent;

impl ChatView {
    /// Render git panel action buttons
    pub fn render_git_panel_actions(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        let git_info = self.git_info.as_ref()?;

        // Extract listener before div chain
        let refresh_status = cx.listener(|this, _, _window, cx| {
            this.refresh_git_status(cx);
        });

        // Copy theme colors for move closures
        let surface_hover = theme.colors.surface_hover;
        let text_color = theme.colors.text;
        let success = theme.colors.success;
        let accent = theme.colors.accent;
        let warning = theme.colors.warning;

        // Clone for move closure
        let branch_to_copy = git_info.branch.clone();
        let has_changes = git_info.staged_count > 0 || git_info.unstaged_count > 0 || git_info.untracked_count > 0;

        Some(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .pt_2()
                .border_t_1()
                .border_color(theme.colors.border)
                // Top row: Basic actions
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .id("git-refresh-btn")
                                .flex()
                                .items_center()
                                .gap_1()
                                .px_2()
                                .py_1()
                                .rounded_md()
                                .cursor_pointer()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .hover(move |s| s.bg(surface_hover).text_color(text_color))
                                .on_click(refresh_status)
                                .child("🔄")
                                .child("Refresh")
                        )
                        .child(
                            div()
                                .id("git-copy-branch-btn")
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
                                .on_click(move |_, _window, cx| {
                                    cx.write_to_clipboard(gpui::ClipboardItem::new_string(branch_to_copy.clone()));
                                })
                                .child("📋")
                                .child("Copy branch")
                        )
                )
                // Bottom row: Claude Code Git skills
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .child("Skills:")
                        )
                        // Commit button (highlighted if has changes)
                        .child({
                            let bg = if has_changes { success.opacity(0.15) } else { gpui::transparent_black() };
                            let border = if has_changes { success.opacity(0.3) } else { theme.colors.border };
                            let color = if has_changes { success } else { theme.colors.text_muted };
                            let hover_bg = success.opacity(0.25);
                            div()
                                .id("git-commit-skill")
                                .flex()
                                .items_center()
                                .gap_1()
                                .px_2()
                                .py_1()
                                .rounded_md()
                                .cursor_pointer()
                                .text_xs()
                                .bg(bg)
                                .border_1()
                                .border_color(border)
                                .text_color(color)
                                .hover(move |s| s.bg(hover_bg))
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.toggle_git_panel(cx);
                                    cx.emit(ChatViewEvent::Submit("/commit".to_string()));
                                }))
                                .child("📦")
                                .child("Commit")
                        })
                        // Create PR button
                        .child({
                            let hover_bg = accent.opacity(0.2);
                            div()
                                .id("git-pr-skill")
                                .flex()
                                .items_center()
                                .gap_1()
                                .px_2()
                                .py_1()
                                .rounded_md()
                                .cursor_pointer()
                                .text_xs()
                                .bg(accent.opacity(0.1))
                                .border_1()
                                .border_color(accent.opacity(0.2))
                                .text_color(accent)
                                .hover(move |s| s.bg(hover_bg))
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.toggle_git_panel(cx);
                                    cx.emit(ChatViewEvent::Submit("/create-pr".to_string()));
                                }))
                                .child("🔀")
                                .child("Create PR")
                        })
                        // Review button
                        .child({
                            let hover_bg = warning.opacity(0.2);
                            div()
                                .id("git-review-skill")
                                .flex()
                                .items_center()
                                .gap_1()
                                .px_2()
                                .py_1()
                                .rounded_md()
                                .cursor_pointer()
                                .text_xs()
                                .bg(warning.opacity(0.1))
                                .border_1()
                                .border_color(warning.opacity(0.2))
                                .text_color(warning)
                                .hover(move |s| s.bg(hover_bg))
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.toggle_git_panel(cx);
                                    cx.emit(ChatViewEvent::Submit("/review".to_string()));
                                }))
                                .child("👀")
                                .child("Review")
                        })
                        // Merge button
                        .child(
                            div()
                                .id("git-merge-skill")
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
                                    this.toggle_git_panel(cx);
                                    cx.emit(ChatViewEvent::Submit("/merge".to_string()));
                                }))
                                .child("🔗")
                                .child("Merge")
                        )
                )
        )
    }
}
