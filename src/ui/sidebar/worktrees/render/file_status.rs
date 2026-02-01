//! File status rendering (staged and unstaged files)

use gpui::prelude::*;
use gpui::*;

use super::super::types::{WorktreePanel, WorktreePanelEvent};
use crate::app::theme::Theme;
use crate::git::status::FileStatusKind;

/// Render quick action buttons for git operations
fn render_quick_actions(
    theme: &Theme,
    has_staged: bool,
    has_unstaged: bool,
    cx: &mut Context<WorktreePanel>,
) -> impl IntoElement {
    div()
        .px_2()
        .py_2()
        .flex()
        .flex_wrap()
        .gap_1()
        .border_b_1()
        .border_color(theme.colors.border.opacity(0.5))
        // Commit button - only show when staged files exist
        .when(has_staged, |d| {
            let accent = theme.colors.accent;
            d.child(
                div()
                    .id("git-action-commit")
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .bg(accent.opacity(0.15))
                    .border_1()
                    .border_color(accent.opacity(0.3))
                    .text_xs()
                    .text_color(accent)
                    .flex()
                    .items_center()
                    .gap_1()
                    .hover(move |s| s.bg(accent.opacity(0.25)).border_color(accent.opacity(0.5)))
                    .on_click(cx.listener(|_this, _, _window, cx| {
                        cx.emit(WorktreePanelEvent::SendSkillCommand("/commit".to_string()));
                    }))
                    .child("Commit"),
            )
        })
        // Review button - show when there are any changes
        .when(has_staged || has_unstaged, |d| {
            let info = theme.colors.info;
            d.child(
                div()
                    .id("git-action-review")
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .bg(info.opacity(0.15))
                    .border_1()
                    .border_color(info.opacity(0.3))
                    .text_xs()
                    .text_color(info)
                    .flex()
                    .items_center()
                    .gap_1()
                    .hover(move |s| s.bg(info.opacity(0.25)).border_color(info.opacity(0.5)))
                    .on_click(cx.listener(|_this, _, _window, cx| {
                        cx.emit(WorktreePanelEvent::SendSkillCommand("/review".to_string()));
                    }))
                    .child("Review"),
            )
        })
        // Create PR button
        .when(has_staged, |d| {
            let success = theme.colors.success;
            d.child(
                div()
                    .id("git-action-create-pr")
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .bg(success.opacity(0.15))
                    .border_1()
                    .border_color(success.opacity(0.3))
                    .text_xs()
                    .text_color(success)
                    .flex()
                    .items_center()
                    .gap_1()
                    .hover(move |s| {
                        s.bg(success.opacity(0.25))
                            .border_color(success.opacity(0.5))
                    })
                    .on_click(cx.listener(|_this, _, _window, cx| {
                        cx.emit(WorktreePanelEvent::SendSkillCommand(
                            "/create-pr".to_string(),
                        ));
                    }))
                    .child("Create PR"),
            )
        })
        // Refactor button - when there are changes
        .when(has_unstaged, |d| {
            let warning = theme.colors.warning;
            d.child(
                div()
                    .id("git-action-refactor")
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .cursor_pointer()
                    .bg(warning.opacity(0.15))
                    .border_1()
                    .border_color(warning.opacity(0.3))
                    .text_xs()
                    .text_color(warning)
                    .flex()
                    .items_center()
                    .gap_1()
                    .hover(move |s| {
                        s.bg(warning.opacity(0.25))
                            .border_color(warning.opacity(0.5))
                    })
                    .on_click(cx.listener(|_this, _, _window, cx| {
                        cx.emit(WorktreePanelEvent::SendSkillCommand(
                            "/refactor".to_string(),
                        ));
                    }))
                    .child("Refactor"),
            )
        })
}

pub(crate) fn render_file_status(
    _panel: &WorktreePanel,
    theme: &Theme,
    staged_files: Vec<(String, char, Option<FileStatusKind>)>,
    unstaged_files: Vec<(String, char, Option<FileStatusKind>)>,
    cx: &mut Context<WorktreePanel>,
) -> impl IntoElement {
    let has_staged = !staged_files.is_empty();
    let has_unstaged = !unstaged_files.is_empty();

    div()
        .px_2()
        .py_2()
        // Quick action buttons at the top
        .child(render_quick_actions(theme, has_staged, has_unstaged, cx))
        // Staged files
        .when(!staged_files.is_empty(), |d| {
            d.child(
                div()
                    .mb_2()
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.colors.success)
                            .child(format!("STAGED ({})", staged_files.len())),
                    )
                    .children(staged_files.into_iter().map(|(path, status_char, _)| {
                        let path_clone = path.clone();
                        let surface_hover = theme.colors.surface_hover;
                        let text_color = theme.colors.text;
                        let success_color = theme.colors.success;

                        let on_click = cx.listener(move |_this, _, _window, cx| {
                            cx.emit(WorktreePanelEvent::FileClicked(path_clone.clone()));
                        });

                        div()
                            .id(ElementId::Name(format!("staged-{}", path).into()))
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .hover(move |style| style.bg(surface_hover))
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .gap_2()
                            .on_click(on_click)
                            .child(
                                div()
                                    .w(px(16.0))
                                    .text_xs()
                                    .font_family("JetBrains Mono")
                                    .text_color(success_color)
                                    .child(status_char.to_string()),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .text_xs()
                                    .text_color(text_color)
                                    .overflow_hidden()
                                    .text_ellipsis()
                                    .child(path),
                            )
                    })),
            )
        })
        // Unstaged files
        .when(!unstaged_files.is_empty(), |d| {
            d.child(
                div()
                    .mb_2()
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.colors.warning)
                            .child(format!("CHANGES ({})", unstaged_files.len())),
                    )
                    .children(unstaged_files.into_iter().map(|(path, status_char, _)| {
                        let path_clone = path.clone();
                        let surface_hover = theme.colors.surface_hover;
                        let text_color = theme.colors.text;
                        let warning_color = theme.colors.warning;

                        let on_click = cx.listener(move |_this, _, _window, cx| {
                            cx.emit(WorktreePanelEvent::FileClicked(path_clone.clone()));
                        });

                        div()
                            .id(ElementId::Name(format!("unstaged-{}", path).into()))
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .hover(move |style| style.bg(surface_hover))
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .gap_2()
                            .on_click(on_click)
                            .child(
                                div()
                                    .w(px(16.0))
                                    .text_xs()
                                    .font_family("JetBrains Mono")
                                    .text_color(warning_color)
                                    .child(status_char.to_string()),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .text_xs()
                                    .text_color(text_color)
                                    .overflow_hidden()
                                    .text_ellipsis()
                                    .child(path),
                            )
                    })),
            )
        })
}
