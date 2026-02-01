//! Permissions panel footer component

use gpui::prelude::*;
use gpui::*;

use super::super::super::core::ChatView;

pub fn render_footer(
    theme: &crate::app::theme::Theme,
    is_empty: bool,
    has_multiple: bool,
    cx: &mut Context<ChatView>,
) -> impl IntoElement {
    // Copy theme colors for move closures
    let success_bg_1 = theme.colors.success.opacity(0.1);
    let success_bg_2 = theme.colors.success.opacity(0.2);
    let success_border_1 = theme.colors.success.opacity(0.3);
    let success_border_2 = theme.colors.success.opacity(0.5);
    let success_color = theme.colors.success;

    let error_bg_1 = theme.colors.error.opacity(0.1);
    let error_bg_2 = theme.colors.error.opacity(0.2);
    let error_border_1 = theme.colors.error.opacity(0.3);
    let error_border_2 = theme.colors.error.opacity(0.5);
    let error_color = theme.colors.error;

    // Extract listeners before div chain
    let approve_all_listener = cx.listener(|this, _, _window, cx| {
        this.approve_all_permissions(cx);
    });

    let deny_all_listener = cx.listener(|this, _, _window, cx| {
        this.deny_all_permissions(cx);
    });

    div()
        .px_4()
        .py_2()
        .border_t_1()
        .border_color(theme.colors.border)
        .flex()
        .items_center()
        .justify_between()
        // Left side - hint text
        .child(
            div()
                .text_xs()
                .text_color(theme.colors.text_muted)
                .when(is_empty, |d| d.child("No pending requests"))
                .when(!is_empty, |d| {
                    d.child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child("⌨️")
                            .child("Press A to approve all, D to deny all"),
                    )
                }),
        )
        // Right side - bulk action buttons (when multiple permissions)
        .when(has_multiple, move |d| {
            d.child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Approve All button
                    .child(
                        div()
                            .id("approve-all-perms")
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_3()
                            .py(px(4.0))
                            .rounded_md()
                            .bg(success_bg_1)
                            .border_1()
                            .border_color(success_border_1)
                            .cursor_pointer()
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(success_color)
                            .hover(move |s| s.bg(success_bg_2).border_color(success_border_2))
                            .on_click(approve_all_listener)
                            .child("✓✓")
                            .child("Approve All"),
                    )
                    // Deny All button
                    .child(
                        div()
                            .id("deny-all-perms")
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_3()
                            .py(px(4.0))
                            .rounded_md()
                            .bg(error_bg_1)
                            .border_1()
                            .border_color(error_border_1)
                            .cursor_pointer()
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(error_color)
                            .hover(move |s| s.bg(error_bg_2).border_color(error_border_2))
                            .on_click(deny_all_listener)
                            .child("××")
                            .child("Deny All"),
                    ),
            )
        })
}
