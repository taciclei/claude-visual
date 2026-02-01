//! LSP Status Bar Rendering
//!
//! Render implementation for the LSP status bar.

use gpui::*;
use gpui::prelude::*;

use super::{LspStatusBar, types::{default_colors, LspStatusBarEvent}};

impl Render for LspStatusBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = default_colors();
        let total_errors = self.total_errors();
        let total_warnings = self.total_warnings();
        let running_count = self.running_count();
        let diagnostics_visible = self.diagnostics_visible;

        div()
            .flex()
            .items_center()
            .gap_3()
            .h(px(24.0))
            .px_3()
            .bg(colors.surface)
            .border_t_1()
            .border_color(colors.border)
            // Errors/Warnings indicator (clickable to show diagnostics)
            .child(
                div()
                    .id("diagnostics-toggle")
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_2()
                    .py_0p5()
                    .rounded_sm()
                    .cursor_pointer()
                    .when(diagnostics_visible, |this| {
                this.bg(colors.selection)
                    })
                    .hover(|style| style.bg(colors.hover))
                    .on_click(cx.listener(|_this, _, _window, cx| {
                        cx.emit(LspStatusBarEvent::ToggleDiagnostics);
                    }))
                    // Error count
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(colors.error)
                                    .child("✕"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(if total_errors > 0 {
                                        colors.error
                                    } else {
                                        colors.text_muted
                                    })
                                    .child(total_errors.to_string()),
                            ),
                    )
                    // Warning count
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(colors.warning)
                                    .child("⚠"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(if total_warnings > 0 {
                                        colors.warning
                                    } else {
                                        colors.text_muted
                                    })
                                    .child(total_warnings.to_string()),
                            ),
                    ),
            )
            // Separator
            .child(
                div()
                    .w(px(1.0))
                    .h(px(14.0))
                    .bg(colors.border),
            )
            // Running servers
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .children(self.servers.values().map(|status| {
                        let language = status.language;
                        let is_running = status.is_running;
                        let has_errors = status.error_count > 0;
                        let icon = Self::language_icon(language);

                        div()
                            .id(ElementId::Name(format!("lsp-{:?}", language).into()))
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_1()
                            .py_0p5()
                            .rounded_sm()
                            .cursor_pointer()
                            .hover(|style| style.bg(colors.hover))
                            .on_click({
                                cx.listener(move |_this, _, _window, cx| {
                                    cx.emit(LspStatusBarEvent::ServerClicked(language));
                                })
                            })
                            // Status indicator
                            .child(
                                div()
                                    .w(px(6.0))
                                    .h(px(6.0))
                                    .rounded_full()
                                    .bg(if !is_running {
                                        colors.text_muted
                                    } else if has_errors {
                                        colors.error
                                    } else {
                                        colors.success
                                    }),
                            )
                            // Language icon
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(if is_running {
                                        colors.text
                                    } else {
                                        colors.text_muted
                                    })
                                    .child(icon),
                            )
                    })),
            )
            // Running count (if any)
            .when(running_count > 0, |this| {
                this.child(
                    div()
                        .text_xs()
                        .text_color(colors.text_muted)
                        .child(format!("{} server{}", running_count, if running_count == 1 { "" } else { "s" })),
                )
            })
    }
}
