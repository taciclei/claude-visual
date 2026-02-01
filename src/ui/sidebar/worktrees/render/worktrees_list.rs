//! Worktrees list rendering with context menu

use gpui::prelude::*;
use gpui::*;

use super::super::types::WorktreePanel;
use crate::app::theme::Theme;

pub(crate) fn render_worktrees_list(
    panel: &WorktreePanel,
    theme: &Theme,
    worktree_items: Vec<(usize, String, String, bool, bool, bool)>,
    cx: &mut Context<WorktreePanel>,
) -> impl IntoElement {
    div()
        .px_2()
        .py_2()
        .border_t_1()
        .border_color(theme.colors.border)
        .child(
            div()
                .px_2()
                .py_1()
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.colors.text_muted)
                .child(format!("WORKTREES ({})", worktree_items.len())),
        )
        .children(worktree_items.into_iter().map(
            |(idx, branch, _path, is_main, is_locked, is_selected)| {
                let bg_color = if is_selected {
                    theme.colors.accent.opacity(0.2)
                } else {
                    theme.colors.surface
                };
                let has_context_menu = panel.context_menu_worktree == Some(idx);

                // Copy theme colors for move closures
                let surface_hover = theme.colors.surface_hover;
                let accent_color = theme.colors.accent;
                let text_color = theme.colors.text;
                let success_color = theme.colors.success;
                let warning_color = theme.colors.warning;
                let error_color = theme.colors.error;
                let border_color = theme.colors.border;
                let text_muted = theme.colors.text_muted;

                // Extract listeners before div builder chain
                let on_click_listener = cx.listener(move |this, _, _window, cx| {
                    this.hide_context_menu(cx);
                    this.select_worktree(idx, cx);
                });

                let on_right_click_listener = cx.listener(move |this, _, _window, cx| {
                    this.show_context_menu(idx, cx);
                });

                let copy_branch_listener = cx.listener(move |this, _, _window, cx| {
                    this.copy_branch_name(idx, cx);
                });

                let copy_path_listener = cx.listener(move |this, _, _window, cx| {
                    this.copy_worktree_path(idx, cx);
                });

                let delete_listener = cx.listener(move |this, _, _window, cx| {
                    this.request_delete_worktree(idx, cx);
                });

                div()
                    .id(ElementId::Name(format!("worktree-{}", idx).into()))
                    .relative()
                    .px_2()
                    .py_1()
                    .rounded_sm()
                    .bg(bg_color)
                    .hover(move |style| style.bg(surface_hover))
                    .cursor_pointer()
                    .on_click(on_click_listener)
                    .on_mouse_down(MouseButton::Right, on_right_click_listener)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(if is_selected {
                                        accent_color
                                    } else {
                                        text_color
                                    })
                                    .child(branch.clone()),
                            )
                            .when(is_main, |d| {
                                d.child(
                                    div()
                                        .px_1()
                                        .rounded_sm()
                                        .bg(success_color.opacity(0.2))
                                        .text_xs()
                                        .text_color(success_color)
                                        .child("main"),
                                )
                            })
                            .when(is_locked, |d| {
                                d.child(
                                    div()
                                        .px_1()
                                        .rounded_sm()
                                        .bg(warning_color.opacity(0.2))
                                        .text_xs()
                                        .text_color(warning_color)
                                        .child("locked"),
                                )
                            }),
                    )
                    // Context menu
                    .when(has_context_menu, |d| {
                        d.child(
                            div()
                                .id(ElementId::Name(format!("worktree-menu-{}", idx).into()))
                                .absolute()
                                .left_full()
                                .top_0()
                                .ml_1()
                                .w(px(160.0))
                                .rounded_md()
                                .bg(theme.colors.surface)
                                .border_1()
                                .border_color(border_color)
                                .shadow_lg()
                                .p_1()
                                .on_mouse_down(MouseButton::Left, |_, _window, _cx| {})
                                // Copy branch name
                                .child(
                                    div()
                                        .id(ElementId::Name(
                                            format!("wt-copy-branch-{}", idx).into(),
                                        ))
                                        .px_2()
                                        .py_1()
                                        .rounded_sm()
                                        .cursor_pointer()
                                        .text_xs()
                                        .text_color(text_color)
                                        .hover(move |s| s.bg(surface_hover))
                                        .on_click(copy_branch_listener)
                                        .child("Copy branch name"),
                                )
                                // Copy path
                                .child(
                                    div()
                                        .id(ElementId::Name(format!("wt-copy-path-{}", idx).into()))
                                        .px_2()
                                        .py_1()
                                        .rounded_sm()
                                        .cursor_pointer()
                                        .text_xs()
                                        .text_color(text_color)
                                        .hover(move |s| s.bg(surface_hover))
                                        .on_click(copy_path_listener)
                                        .child("Copy path"),
                                )
                                // Divider
                                .child(div().my_1().h(px(1.0)).bg(border_color))
                                // Delete (only for non-main worktrees)
                                .when(!is_main, |d| {
                                    d.child(
                                        div()
                                            .id(ElementId::Name(
                                                format!("wt-delete-{}", idx).into(),
                                            ))
                                            .px_2()
                                            .py_1()
                                            .rounded_sm()
                                            .cursor_pointer()
                                            .text_xs()
                                            .text_color(error_color)
                                            .hover(move |s| s.bg(error_color.opacity(0.1)))
                                            .on_click(delete_listener)
                                            .child("Delete worktree"),
                                    )
                                })
                                // Main worktree can't be deleted
                                .when(is_main, |d| {
                                    d.child(
                                        div()
                                            .px_2()
                                            .py_1()
                                            .text_xs()
                                            .text_color(text_muted)
                                            .child("Cannot delete main"),
                                    )
                                }),
                        )
                    })
            },
        ))
}
