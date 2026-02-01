//! Quick actions render function

use super::super::super::core::ChatView;
use super::super::super::types::ChatViewEvent;
use gpui::prelude::*;
use gpui::*;

impl ChatView {
    pub fn render_quick_actions(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Div {
        // Quick action button definitions: (label, command, description, icon)
        let quick_actions = [
            ("Resume", "/resume", "Continue previous session", "↩"),
            ("Clear", "/clear", "Start fresh conversation", "✨"),
            ("Usage", "/usage", "Token usage & costs", "📊"),
            ("Compact", "/compact", "Compress context", "📦"),
            ("Memory", "/memory", "Persistent memory", "🧠"),
            ("Model", "/model", "Switch AI model", "🤖"),
            ("Help", "/help", "Available commands", "❓"),
            ("Doctor", "/doctor", "Run diagnostics", "🩺"),
        ];

        div()
            .w_full()
            .flex()
            .flex_wrap()
            .gap_2()
            .justify_center()
            .children(quick_actions.iter().map(|(label, cmd, desc, icon)| {
                let cmd_str = cmd.to_string();
                div()
                    .id(SharedString::from(format!("qa-{}", label)))
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_1()
                    .px_4()
                    .py_3()
                    .rounded_lg()
                    .bg(theme.colors.surface)
                    .border_1()
                    .border_color(theme.colors.border)
                    .cursor_pointer()
                    .hover(|s| {
                        s.bg(theme.colors.surface_hover)
                            .border_color(theme.colors.accent.opacity(0.5))
                    })
                    .on_click(cx.listener(move |this, _, _window, cx| {
                        // Set the command in the input and submit
                        this.input.update(cx, |input, cx| {
                            input.set_text(cmd_str.clone(), cx);
                        });
                        cx.emit(ChatViewEvent::Submit(cmd_str.clone()));
                    }))
                    .child(div().text_lg().child(*icon))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.colors.text)
                            .child(*label),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child(*desc),
                    )
            }))
    }
}
