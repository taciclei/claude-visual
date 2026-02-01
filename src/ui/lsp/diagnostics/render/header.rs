//! Header rendering for diagnostics panel

use gpui::*;
use gpui::prelude::*;

use crate::lsp::protocol::DiagnosticSeverity;
use crate::ui::lsp::diagnostics::core::DiagnosticsPanel;
use crate::ui::lsp::diagnostics::types::SimpleColors;

pub fn render_header(
    panel: &DiagnosticsPanel,
    cx: &mut Context<DiagnosticsPanel>,
    colors: &SimpleColors,
) -> impl IntoElement {
    let error_count = panel.total_errors();
    let warning_count = panel.total_warnings();
    let colors_hover = colors.hover;
    let colors_selection = colors.selection;
    let colors_text = colors.text;
    let colors_text_muted = colors.text_muted;
    let colors_error = colors.error;
    let colors_warning = colors.warning;

    let toggle_click = cx.listener(|this, _, _window, cx| {
        this.toggle_expanded(cx);
    });

    let filter_all_click = cx.listener(|this, _, _window, cx| {
        this.set_filter(None, cx);
    });

    let filter_errors_click = cx.listener(|this, _, _window, cx| {
        this.set_filter(Some(DiagnosticSeverity::Error), cx);
    });

    let filter_warnings_click = cx.listener(|this, _, _window, cx| {
        this.set_filter(Some(DiagnosticSeverity::Warning), cx);
    });

    div()
        .id("diagnostics-header")
        .flex()
        .items_center()
        .justify_between()
        .px_3()
        .py_2()
        .bg(colors.surface)
        .border_b_1()
        .border_color(colors.border)
        .cursor_pointer()
        .on_click(toggle_click)
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(colors_text)
                        .child(if panel.is_expanded { "▼" } else { "▶" }),
                )
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(colors_text)
                        .child("Problems"),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .child(
                            div()
                                .text_xs()
                                .text_color(colors_error)
                                .child("✕"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(if error_count > 0 {
                                    colors_error
                                } else {
                                    colors_text_muted
                                })
                                .child(error_count.to_string()),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .child(
                            div()
                                .text_xs()
                                .text_color(colors_warning)
                                .child("⚠"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(if warning_count > 0 {
                                    colors_warning
                                } else {
                                    colors_text_muted
                                })
                                .child(warning_count.to_string()),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .child(
                            div()
                                .id("filter-all")
                                .px_2()
                                .py_0p5()
                                .rounded_sm()
                                .cursor_pointer()
                                .when(panel.severity_filter.is_none(), |this| {
                                    this.bg(colors_selection)
                                })
                                .hover(|style| style.bg(colors_hover))
                                .on_click(filter_all_click)
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(colors_text_muted)
                                        .child("All"),
                                ),
                        )
                        .child(
                            div()
                                .id("filter-errors")
                                .px_2()
                                .py_0p5()
                                .rounded_sm()
                                .cursor_pointer()
                                .when(
                                    panel.severity_filter == Some(DiagnosticSeverity::Error),
                                    |this| this.bg(colors_selection),
                                )
                                .hover(|style| style.bg(colors_hover))
                                .on_click(filter_errors_click)
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(colors_error)
                                        .child("Errors"),
                                ),
                        )
                        .child(
                            div()
                                .id("filter-warnings")
                                .px_2()
                                .py_0p5()
                                .rounded_sm()
                                .cursor_pointer()
                                .when(
                                    panel.severity_filter == Some(DiagnosticSeverity::Warning),
                                    |this| this.bg(colors_selection),
                                )
                                .hover(|style| style.bg(colors_hover))
                                .on_click(filter_warnings_click)
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(colors_warning)
                                        .child("Warnings"),
                                ),
                        ),
                ),
        )
}
