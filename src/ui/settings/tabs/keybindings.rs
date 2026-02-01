use super::super::SettingsModal;
use gpui::prelude::*;
use gpui::*;

impl SettingsModal {
    /// Render the keybindings tab
    pub(crate) fn render_keybindings_tab(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let keybindings = self.pending.keybindings.all_bindings();

        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.colors.text)
                            .child("Keyboard Shortcuts"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text_muted)
                            .child("Customize keyboard shortcuts for common actions"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .p_4()
                    .bg(theme.colors.surface)
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.colors.border)
                    .children(keybindings.into_iter().enumerate().map(
                        |(idx, (action, keybinding))| {
                            let action_owned = action.to_string();
                            let keybinding_owned = keybinding.to_string();
                            let theme = theme.clone();

                            div()
                                .id(SharedString::from(format!("keybinding-{}", idx)))
                                .flex()
                                .items_center()
                                .justify_between()
                                .py_2()
                                .px_3()
                                .rounded_md()
                                .hover(|s| s.bg(theme.colors.surface_hover))
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(theme.colors.text)
                                        .child(action_owned.clone()),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .px_2()
                                                .py_1()
                                                .bg(theme.colors.surface_hover)
                                                .rounded_md()
                                                .border_1()
                                                .border_color(theme.colors.border)
                                                .text_xs()
                                                .font_family("JetBrains Mono")
                                                .text_color(theme.colors.text_muted)
                                                .child(self.format_keybinding(&keybinding_owned)),
                                        )
                                        .child(
                                            div()
                                                .id(SharedString::from(format!(
                                                    "edit-keybinding-{}",
                                                    idx
                                                )))
                                                .px_2()
                                                .py_1()
                                                .rounded_md()
                                                .cursor_pointer()
                                                .text_xs()
                                                .text_color(theme.colors.text_muted)
                                                .hover(|s| {
                                                    s.bg(theme.colors.surface_hover)
                                                        .text_color(theme.colors.text)
                                                })
                                                .on_click({
                                                    let action = action_owned.clone();
                                                    cx.listener(move |this, _, _window, cx| {
                                                        this.start_keybinding_edit(&action, cx);
                                                    })
                                                })
                                                .child("Edit"),
                                        ),
                                )
                        },
                    )),
            )
            .child(
                div()
                    .pt_4()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child("Click 'Edit' to change a keybinding. Press Escape to cancel."),
            )
    }

    /// Format a keybinding for display (e.g., "cmd-b" -> "⌘B")
    fn format_keybinding(&self, keybinding: &str) -> String {
        keybinding
            .split('-')
            .map(|part| match part.to_lowercase().as_str() {
                "cmd" => "⌘".to_string(),
                "ctrl" => "⌃".to_string(),
                "alt" | "option" => "⌥".to_string(),
                "shift" => "⇧".to_string(),
                "enter" | "return" => "↵".to_string(),
                "tab" => "⇥".to_string(),
                "escape" | "esc" => "⎋".to_string(),
                "space" => "␣".to_string(),
                "backspace" => "⌫".to_string(),
                "delete" => "⌦".to_string(),
                "up" => "↑".to_string(),
                "down" => "↓".to_string(),
                "left" => "←".to_string(),
                "right" => "→".to_string(),
                other => other.to_uppercase(),
            })
            .collect::<Vec<_>>()
            .join("")
    }

    /// Start editing a keybinding
    fn start_keybinding_edit(&mut self, action: &str, cx: &mut Context<Self>) {
        // For now, just log that we're starting to edit
        // In a full implementation, this would open a dialog to capture the new keybinding
        eprintln!("Starting keybinding edit for: {}", action);
        cx.notify();
    }
}
