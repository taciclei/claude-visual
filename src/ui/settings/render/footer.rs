//! Settings modal footer with action buttons

use super::super::core::SettingsModal;
use crate::app::theme::Theme;
use gpui::prelude::*;
use gpui::*;

impl SettingsModal {
    pub(super) fn render_footer(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let on_reset = cx.listener(|this, _, _window, cx| {
            this.show_reset_confirmation(cx);
        });
        let on_export = cx.listener(|this, _, _window, cx| {
            this.show_export(cx);
        });
        let on_import = cx.listener(|this, _, _window, cx| {
            this.show_import(cx);
        });
        let on_cancel = cx.listener(|this, _, _window, cx| {
            this.dismiss(cx);
        });
        let on_save = cx.listener(|this, _, _window, cx| {
            if this.has_changes {
                this.save(cx);
            }
        });

        let error_color = theme.colors.error;
        let surface_hover = theme.colors.surface_hover;
        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;
        let border_color = theme.colors.border;
        let accent_color = theme.colors.accent;

        div()
            .flex()
            .items_center()
            .justify_between()
            .px_4()
            .py_3()
            .border_t_1()
            .border_color(theme.colors.border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_4()
                    // Reset to defaults button
                    .child(
                        div()
                            .id("settings-reset")
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .text_xs()
                            .text_color(error_color)
                            .border_1()
                            .border_color(error_color.opacity(0.3))
                            .hover(move |s| s.bg(error_color.opacity(0.1)))
                            .on_click(on_reset)
                            .child("Reset to Defaults"),
                    )
                    // Export button
                    .child(
                        div()
                            .id("settings-export")
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .text_xs()
                            .text_color(text_muted)
                            .border_1()
                            .border_color(border_color)
                            .hover(move |s| s.bg(surface_hover).text_color(text_color))
                            .on_click(on_export)
                            .child("Export"),
                    )
                    // Import button
                    .child(
                        div()
                            .id("settings-import")
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .text_xs()
                            .text_color(text_muted)
                            .border_1()
                            .border_color(border_color)
                            .hover(move |s| s.bg(surface_hover).text_color(text_color))
                            .on_click(on_import)
                            .child("Import"),
                    )
                    // Unsaved changes indicator
                    .child(
                        div()
                            .text_xs()
                            .text_color(text_muted)
                            .when(self.has_changes, |d| d.child("You have unsaved changes"))
                            .when(!self.has_changes, |d| d.child("")),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    // Cancel button
                    .child(
                        div()
                            .id("settings-cancel")
                            .px_4()
                            .py_2()
                            .rounded_md()
                            .cursor_pointer()
                            .text_sm()
                            .text_color(text_muted)
                            .hover(move |s| s.bg(surface_hover).text_color(text_color))
                            .on_click(on_cancel)
                            .child("Cancel"),
                    )
                    // Save button
                    .child(
                        div()
                            .id("settings-save")
                            .px_4()
                            .py_2()
                            .rounded_md()
                            .cursor_pointer()
                            .text_sm()
                            .when(self.has_changes, |d| {
                                d.bg(accent_color)
                                    .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                                    .hover(|s| s.opacity(0.9))
                            })
                            .when(!self.has_changes, |d| {
                                d.bg(border_color).text_color(text_muted)
                            })
                            .on_click(on_save)
                            .child("Save Changes"),
                    ),
            )
    }
}
