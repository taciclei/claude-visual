//! Send button rendering

use gpui::prelude::*;
use gpui::*;

use crate::app::theme::Theme;

use super::ChatInput;

impl ChatInput {
    /// Render send button with keyboard hint
    pub(super) fn render_send_button(
        &self,
        theme: &Theme,
        can_submit: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_1()
            .child(
                div()
                    .id("send-button")
                    .flex_shrink_0()
                    .size(px(40.0))
                    .rounded_lg()
                    .bg(if can_submit {
                        theme.colors.accent
                    } else {
                        theme.colors.border
                    })
                    .when(can_submit, |d| {
                        d.hover(|style| style.bg(theme.colors.accent_hover))
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.submit(cx);
                            }))
                    })
                    .when(!can_submit, |d| d.cursor_not_allowed())
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_lg()
                            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                            .child("↑"),
                    ),
            )
            // Keyboard hint below button
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .font_family("monospace")
                    .child("⏎"),
            )
    }
}
