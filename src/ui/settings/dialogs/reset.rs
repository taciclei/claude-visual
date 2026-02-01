use super::super::SettingsModal;
use gpui::prelude::*;
use gpui::*;

impl SettingsModal {
    /// Render the reset confirmation dialog
    pub(crate) fn render_reset_confirmation_dialog(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        // Extract listeners before div chains
        let on_cancel = cx.listener(|this, _, _window, cx| {
            this.hide_reset_confirmation(cx);
        });
        let on_confirm = cx.listener(|this, _, _window, cx| {
            this.reset_to_defaults(cx);
        });

        // Copy theme colors for move closures
        let surface_hover = theme.colors.surface_hover;
        let text = theme.colors.text;
        let text_muted = theme.colors.text_muted;

        div()
            .id("reset-confirmation-dialog")
            .absolute()
            .inset_0()
            .bg(theme.colors.background.opacity(0.5))
            .flex()
            .items_center()
            .justify_center()

            .child(
                div()
                    .w(px(400.0))
                    .p_5()
                    .rounded_lg()
                    .bg(theme.colors.surface)
                    .border_1()
                    .border_color(theme.colors.border)
                    .flex()
                    .flex_col()
                    .gap_4()
                    // Prevent clicks from closing
                    .on_mouse_down(MouseButton::Left, |_, _window, cx| {
                    })
                    // Title
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .w_10()
                                    .h_10()
                                    .rounded_full()
                                    .bg(theme.colors.warning.opacity(0.2))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(theme.colors.warning)
                                    .text_lg()
                                    .child("!")
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_base()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.colors.text)
                                            .child("Reset Settings")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(theme.colors.text_muted)
                                            .child("This will restore all settings to their default values.")
                                    )
                            )
                    )
                    // Warning message
                    .child(
                        div()
                            .p_3()
                            .rounded_md()
                            .bg(theme.colors.warning.opacity(0.1))
                            .border_1()
                            .border_color(theme.colors.warning.opacity(0.3))
                            .text_sm()
                            .text_color(theme.colors.warning)
                            .child("Your current settings will be lost. Click \"Save Changes\" after resetting to apply the defaults permanently.")
                    )
                    // Buttons
                    .child(
                        div()
                            .flex()
                            .justify_end()
                            .gap_2()
                            .child(
                                div()
                                    .id("reset-cancel")
                                    .px_4()
                                    .py_2()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_sm()
                                    .text_color(text_muted)
                                    .hover(move |s| {
                                        s.bg(surface_hover)
                                            .text_color(text)
                                    })
                                    .on_click(on_cancel)
                                    .child("Cancel")
                            )
                            .child(
                                div()
                                    .id("reset-confirm")
                                    .px_4()
                                    .py_2()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_sm()
                                    .bg(theme.colors.error)
                                    .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                                    .hover(|s| s.opacity(0.9))
                                    .on_click(on_confirm)
                                    .child("Reset")
                            )
                    )
            )
    }
}
