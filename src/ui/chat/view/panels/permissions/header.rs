//! Permissions panel header component

use gpui::prelude::*;
use gpui::*;

use super::super::super::core::ChatView;

pub fn render_header(
    theme: &crate::app::theme::Theme,
    permissions_count: usize,
    cx: &mut Context<ChatView>,
) -> impl IntoElement {
    // Copy theme colors for move closures
    let surface_hover = theme.colors.surface_hover;
    let text_muted = theme.colors.text_muted;

    // Extract listener before div chain
    let close_listener = cx.listener(|this, _, _window, cx| {
        this.toggle_permissions_panel(cx);
    });

    div()
        .px_4()
        .py_3()
        .border_b_1()
        .border_color(theme.colors.border)
        .bg(theme.colors.warning.opacity(0.05))
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(div().text_base().child("🔐"))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.colors.text)
                        .child("Permission Requests"),
                )
                .child(
                    div()
                        .px_2()
                        .py_px()
                        .rounded_full()
                        .bg(theme.colors.warning.opacity(0.2))
                        .text_xs()
                        .text_color(theme.colors.warning)
                        .child(format!("{}", permissions_count)),
                ),
        )
        .child(
            div()
                .id("close-permissions")
                .px_2()
                .py_1()
                .rounded_md()
                .cursor_pointer()
                .text_sm()
                .text_color(text_muted)
                .hover(move |s| s.bg(surface_hover))
                .on_click(close_listener)
                .child("×"),
        )
}
