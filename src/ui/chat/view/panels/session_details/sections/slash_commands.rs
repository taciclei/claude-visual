//! Slash commands section rendering

use gpui::prelude::*;
use gpui::*;

use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Renders the slash commands section
    pub(crate) fn render_slash_commands_section(
        &self,
        info: &crate::claude::message::SessionInfo,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.colors.text_muted)
                            .child(format!("SLASH COMMANDS ({})", info.slash_commands.len())),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted.opacity(0.6))
                            .child("Click to insert"),
                    ),
            )
            .child(
                div()
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .bg(theme.colors.background)
                    .flex()
                    .flex_wrap()
                    .gap_1()
                    .children(
                        info.slash_commands
                            .iter()
                            .take(15)
                            .enumerate()
                            .map(|(idx, cmd)| {
                                let cmd_clone = cmd.clone();
                                let success_bg = theme.colors.success.opacity(0.1);
                                let success_hover = theme.colors.success.opacity(0.2);
                                let success_color = theme.colors.success;

                                let on_click_listener = cx.listener(move |this, _, _window, cx| {
                                    this.insert_slash_command(&cmd_clone, cx);
                                });

                                div()
                                    .id(ElementId::Name(format!("slash-cmd-{}", idx).into()))
                                    .px_2()
                                    .py(px(2.0))
                                    .rounded_sm()
                                    .bg(success_bg)
                                    .text_xs()
                                    .font_family("monospace")
                                    .text_color(success_color)
                                    .cursor_pointer()
                                    .hover(move |s| s.bg(success_hover))
                                    .on_click(on_click_listener)
                                    .child(format!("/{}", cmd))
                            }),
                    )
                    .when(info.slash_commands.len() > 15, |d| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .child(format!("+{} more", info.slash_commands.len() - 15)),
                        )
                    }),
            )
    }
}
