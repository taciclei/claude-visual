use gpui::*;
use gpui::prelude::*;
use crate::ui::pct;
use super::super::SettingsModal;

impl SettingsModal {
    /// Render the import/export dialog
    pub(crate) fn render_import_export_dialog(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let is_import = self.import_mode;
        let title = if is_import { "Import Settings" } else { "Export Settings" };
        let description = if is_import {
            "Paste JSON settings below to import"
        } else {
            "Copy the JSON below to export your settings"
        };
        let has_error = self.import_export_error.is_some();
        let text_content = self.import_export_text.clone();
        let error_msg = self.import_export_error.clone();

        // Extract listeners before div chains
        let on_copy = cx.listener(|this, _, _window, cx| {
            this.copy_to_clipboard(cx);
        });
        let on_paste = cx.listener(|this, _, _window, cx| {
            this.paste_from_clipboard(cx);
        });
        let on_cancel = cx.listener(|this, _, _window, cx| {
            this.hide_import_export(cx);
        });
        let on_apply = cx.listener(|this, _, _window, cx| {
            this.apply_import(cx);
        });

        // Copy theme colors for move closures
        let accent = theme.colors.accent;
        let accent_opacity_01 = theme.colors.accent.opacity(0.1);
        let accent_opacity_03 = theme.colors.accent.opacity(0.3);
        let surface_hover = theme.colors.surface_hover;
        let text = theme.colors.text;
        let text_muted = theme.colors.text_muted;

        div()
            .id("import-export-dialog")
            .absolute()
            .inset_0()
            .bg(theme.colors.background.opacity(0.5))
            .flex()
            .items_center()
            .justify_center()

            .child(
                div()
                    .w(px(500.0))
                    .max_h(pct(80.0))
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
                    // Header
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
                                    .bg(theme.colors.accent.opacity(0.2))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_color(theme.colors.accent)
                                    .text_lg()
                                    .child(if is_import { "↓" } else { "↑" })
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
                                            .child(title)
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(theme.colors.text_muted)
                                            .child(description)
                                    )
                            )
                    )
                    // Text area (display-only for now, shows content)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .w_full()
                                    .h(px(200.0))
                                    .p_3()
                                    .rounded_md()
                                    .bg(theme.colors.background)
                                    .border_1()
                                    .border_color(if has_error {
                                        theme.colors.error
                                    } else {
                                        theme.colors.border
                                    })
                                    .id("scroll-json-editor")
                                    .overflow_y_scroll()
                                    .text_xs()
                                    .font_family("JetBrains Mono")
                                    .text_color(theme.colors.text_muted)
                                    .child(if text_content.is_empty() {
                                        if is_import {
                                            "Paste JSON settings here..."
                                        } else {
                                            "Generating export..."
                                        }.to_string()
                                    } else {
                                        text_content.clone()
                                    }),
                            )
                            // Error message
                            .when(error_msg.is_some(), |d| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.colors.error)
                                        .child(error_msg.unwrap_or_default())
                                )
                            })
                    )
                    // Action buttons
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            // Left side - clipboard buttons
                            .child(
                                div()
                                    .flex()
                                    .gap_2()
                                    .when(!is_import, |d| {
                                        d.child(
                                            div()
                                                .id("export-copy")
                                                .px_3()
                                                .py_1()
                                                .rounded_md()
                                                .cursor_pointer()
                                                .text_xs()
                                                .text_color(accent)
                                                .border_1()
                                                .border_color(accent_opacity_03)
                                                .hover(move |s| s.bg(accent_opacity_01))
                                                .on_click(on_copy)
                                                .child("Copy to Clipboard")
                                        )
                                    })
                                    .when(is_import, |d| {
                                        d.child(
                                            div()
                                                .id("import-paste")
                                                .px_3()
                                                .py_1()
                                                .rounded_md()
                                                .cursor_pointer()
                                                .text_xs()
                                                .text_color(accent)
                                                .border_1()
                                                .border_color(accent_opacity_03)
                                                .hover(move |s| s.bg(accent_opacity_01))
                                                .on_click(on_paste)
                                                .child("Paste from Clipboard")
                                        )
                                    })
                            )
                            // Right side - close/apply buttons
                            .child(
                                div()
                                    .flex()
                                    .gap_2()
                                    .child(
                                        div()
                                            .id("import-export-cancel")
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
                                            .child("Close")
                                    )
                                    .when(is_import, |d| {
                                        d.child(
                                            div()
                                                .id("import-apply")
                                                .px_4()
                                                .py_2()
                                                .rounded_md()
                                                .cursor_pointer()
                                                .text_sm()
                                                .bg(theme.colors.accent)
                                                .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                                                .hover(|s| s.opacity(0.9))
                                                .on_click(on_apply)
                                                .child("Apply Import")
                                        )
                                    })
                            )
                    )
            )
    }
}
