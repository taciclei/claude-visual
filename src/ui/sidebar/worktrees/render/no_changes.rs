//! No changes message rendering

use gpui::prelude::*;
use gpui::*;

use crate::app::theme::Theme;

use super::super::types::WorktreePanelEvent;

pub(crate) fn render_no_changes(
    theme: &Theme,
    cx: &mut Context<super::super::types::WorktreePanel>,
) -> impl IntoElement {
    let accent = theme.colors.accent;
    let info = theme.colors.info;
    let success_color = theme.colors.success;

    div()
        .px_4()
        .py_4()
        .flex()
        .flex_col()
        .items_center()
        .gap_3()
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(div().text_base().child("✓"))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(success_color)
                        .child("Working tree clean"),
                ),
        )
        .child(
            div()
                .text_xs()
                .text_color(theme.colors.text_muted)
                .text_center()
                .child("Ready to start coding!"),
        )
        // Quick skill suggestions
        .child(
            div()
                .pt_2()
                .flex()
                .flex_wrap()
                .justify_center()
                .gap_2()
                // Explore codebase
                .child(
                    div()
                        .id("no-changes-explore")
                        .px_2()
                        .py_1()
                        .rounded_md()
                        .cursor_pointer()
                        .bg(accent.opacity(0.15))
                        .border_1()
                        .border_color(accent.opacity(0.3))
                        .text_xs()
                        .text_color(accent)
                        .hover(move |s| {
                            s.bg(accent.opacity(0.25))
                                .border_color(accent.opacity(0.5))
                        })
                        .on_click(cx.listener(|_this, _, _window, cx| {
                            cx.emit(WorktreePanelEvent::SendSkillCommand(
                                "/explore".to_string(),
                            ));
                        }))
                        .child("🔍 Explore"),
                )
                // Start with APEX
                .child(
                    div()
                        .id("no-changes-apex")
                        .px_2()
                        .py_1()
                        .rounded_md()
                        .cursor_pointer()
                        .bg(info.opacity(0.15))
                        .border_1()
                        .border_color(info.opacity(0.3))
                        .text_xs()
                        .text_color(info)
                        .hover(move |s| s.bg(info.opacity(0.25)).border_color(info.opacity(0.5)))
                        .on_click(cx.listener(|_this, _, _window, cx| {
                            cx.emit(WorktreePanelEvent::SendSkillCommand("/apex".to_string()));
                        }))
                        .child("⚡ APEX"),
                ),
        )
}

pub(crate) fn render_no_git_repo(
    theme: &Theme,
    cx: &mut Context<super::super::types::WorktreePanel>,
) -> impl IntoElement {
    let accent = theme.colors.accent;
    let warning = theme.colors.warning;

    div()
        .py_6()
        .px_4()
        .flex()
        .flex_col()
        .items_center()
        .gap_3()
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(div().text_lg().child("📁"))
                .child(
                    div()
                        .text_sm()
                        .text_color(theme.colors.text_muted)
                        .child("No git repository"),
                ),
        )
        .child(
            div()
                .text_xs()
                .text_color(theme.colors.text_muted)
                .text_center()
                .max_w(px(180.0))
                .child("Select a project with git or explore the codebase"),
        )
        // Quick skill suggestions
        .child(
            div()
                .pt_2()
                .flex()
                .flex_wrap()
                .justify_center()
                .gap_2()
                // Explore codebase
                .child(
                    div()
                        .id("no-git-explore")
                        .px_2()
                        .py_1()
                        .rounded_md()
                        .cursor_pointer()
                        .bg(accent.opacity(0.15))
                        .border_1()
                        .border_color(accent.opacity(0.3))
                        .text_xs()
                        .text_color(accent)
                        .hover(move |s| {
                            s.bg(accent.opacity(0.25))
                                .border_color(accent.opacity(0.5))
                        })
                        .on_click(cx.listener(|_this, _, _window, cx| {
                            cx.emit(WorktreePanelEvent::SendSkillCommand(
                                "/explore".to_string(),
                            ));
                        }))
                        .child("🔍 Explore"),
                )
                // Initialize git
                .child(
                    div()
                        .id("no-git-init")
                        .px_2()
                        .py_1()
                        .rounded_md()
                        .cursor_pointer()
                        .bg(warning.opacity(0.15))
                        .border_1()
                        .border_color(warning.opacity(0.3))
                        .text_xs()
                        .text_color(warning)
                        .hover(move |s| {
                            s.bg(warning.opacity(0.25))
                                .border_color(warning.opacity(0.5))
                        })
                        .on_click(cx.listener(|_this, _, _window, cx| {
                            cx.emit(WorktreePanelEvent::SendSkillCommand(
                                "Initialize a git repository in this project".to_string(),
                            ));
                        }))
                        .child("🔧 Init Git"),
                ),
        )
}
