//! Command display component

use gpui::prelude::*;
use gpui::*;

/// Command display (for terminal commands)
#[derive(Clone)]
pub struct Command {
    pub(crate) command: String,
    pub(crate) shell: Option<String>,
    copyable: bool,
}

impl Command {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            shell: None,
            copyable: true,
        }
    }

    pub fn shell(mut self, shell: impl Into<String>) -> Self {
        self.shell = Some(shell.into());
        self
    }

    pub fn not_copyable(mut self) -> Self {
        self.copyable = false;
        self
    }
}

impl RenderOnce for Command {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bg = hsla(0.0, 0.0, 0.1, 1.0);
        let text = hsla(0.38, 0.7, 0.6, 1.0); // Green for commands
        let prompt = hsla(0.0, 0.0, 0.5, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);

        div()
            .px_3()
            .py_2()
            .bg(bg)
            .rounded(px(6.0))
            .flex()
            .items_center()
            .justify_between()
            .gap_3()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .font_family("monospace")
                    .text_sm()
                    // Prompt
                    .child(
                        div()
                            .text_color(prompt)
                            .child(self.shell.unwrap_or_else(|| "$".to_string())),
                    )
                    // Command
                    .child(div().text_color(text).child(self.command)),
            )
            .when(self.copyable, |d| {
                d.child(
                    div()
                        .size(px(24.0))
                        .rounded(px(4.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_xs()
                        .text_color(text_muted)
                        .cursor_pointer()
                        .hover(|s| {
                            s.bg(hsla(0.0, 0.0, 0.18, 1.0))
                                .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                        })
                        .child("📋"),
                )
            })
    }
}
