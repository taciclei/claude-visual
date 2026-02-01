//! Keyboard shortcut hints and recent commands rendering

use crate::ui::chat::view::core::ChatView;
use crate::ui::chat::view::types::ChatViewEvent;
use gpui::prelude::*;
use gpui::*;

impl ChatView {
    pub(super) fn render_keyboard_hints(
        &self,
        theme: &crate::app::theme::Theme,
    ) -> impl IntoElement {
        let hints = [
            ("⌘K", "palette"),
            ("/", "skills"),
            ("@", "files"),
            ("⌘?", "help"),
        ];

        div()
            .flex()
            .items_center()
            .gap_2()
            .children(hints.iter().map(|(key, label)| {
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(
                        div()
                            .px_1()
                            .py_px()
                            .rounded_sm()
                            .bg(theme.colors.background)
                            .border_1()
                            .border_color(theme.colors.border)
                            .text_xs()
                            .font_family("monospace")
                            .text_color(theme.colors.text_muted)
                            .child(*key),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted.opacity(0.7))
                            .child(*label),
                    )
            }))
    }

    pub(super) fn render_recent_commands(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div().when(!self.recent_commands.is_empty(), |d| {
            d.child(div().w(px(1.0)).h(px(12.0)).bg(theme.colors.border).mx_1())
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted.opacity(0.6))
                                .child("Recent:"),
                        )
                        .children(self.recent_commands.iter().take(3).enumerate().map(
                            |(idx, cmd)| {
                                let cmd_clone = cmd.clone();
                                div()
                                    .id(ElementId::Name(format!("recent-cmd-{}", idx).into()))
                                    .px_1()
                                    .py_px()
                                    .rounded_sm()
                                    .bg(theme.colors.background.opacity(0.5))
                                    .border_1()
                                    .border_color(theme.colors.border.opacity(0.5))
                                    .text_xs()
                                    .font_family("monospace")
                                    .text_color(theme.colors.text_muted)
                                    .cursor_pointer()
                                    .hover(|s| {
                                        s.bg(theme.colors.surface_hover)
                                            .text_color(theme.colors.text)
                                    })
                                    .on_click(cx.listener(move |_this, _, _window, cx| {
                                        cx.emit(ChatViewEvent::Submit(cmd_clone.clone()));
                                    }))
                                    .child(cmd.clone())
                            },
                        )),
                )
        })
    }
}
