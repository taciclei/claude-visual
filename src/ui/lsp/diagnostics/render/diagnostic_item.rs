//! Individual diagnostic item rendering

use std::path::PathBuf;
use gpui::*;
use gpui::prelude::*;

use crate::lsp::protocol::Diagnostic;
use crate::ui::lsp::diagnostics::core::DiagnosticsPanel;
use crate::ui::lsp::diagnostics::types::{SimpleColors, DiagnosticsPanelEvent, severity_icon, severity_color};

pub fn render_diagnostic_items(
    diagnostics: &[&Diagnostic],
    path: &PathBuf,
    cx: &mut Context<DiagnosticsPanel>,
    colors: &SimpleColors,
) -> impl IntoElement {
    let colors_text = colors.text;
    let colors_text_muted = colors.text_muted;
    let colors_hover = colors.hover;
    let colors_accent = colors.accent;

    div()
        .flex()
        .flex_col()
        .children(diagnostics.iter().enumerate().map(|(idx, diagnostic)| {
            let severity = diagnostic.severity;
            let icon = severity_icon(severity);
            let color = severity_color(severity, colors);
            let message = diagnostic.message.clone();
            let line = diagnostic.range.start.line + 1;
            let column = diagnostic.range.start.character + 1;
            let source = diagnostic.source.clone();
            let diag = (*diagnostic).clone();
            let path_clone = path.clone();

            let goto_click = {
                let path = path.clone();
                cx.listener(move |_this, _, _window, cx| {
                    cx.emit(DiagnosticsPanelEvent::GoToLocation {
                        file: path.clone(),
                        line: line - 1,
                        column: column - 1,
                    });
                })
            };

            let fix_click = cx.listener(move |_this, _, _window, cx| {
                cx.emit(DiagnosticsPanelEvent::QuickFix(diag.clone()));
            });

            div()
                .id(ElementId::Name(format!("diag-{}-{}", path_clone.display(), idx).into()))
                .flex()
                .items_start()
                .gap_2()
                .px_3()
                .pl_6()
                .py_1()
                .cursor_pointer()
                .hover(|style| style.bg(colors_hover))
                .on_click(goto_click)
                .child(
                    div()
                        .text_sm()
                        .text_color(color)
                        .child(icon),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .overflow_hidden()
                        .child(
                            div()
                                .text_sm()
                                .text_color(colors_text)
                                .child(message),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(colors_text_muted)
                                        .child(format!("Ln {}, Col {}", line, column)),
                                )
                                .when(source.is_some(), |this| {
                                    this.child(
                                        div()
                                            .text_xs()
                                            .text_color(colors_text_muted)
                                            .child(format!("({})", source.unwrap())),
                                    )
                                }),
                        ),
                )
                .child(
                    div()
                        .id(ElementId::Name(format!("fix-{}-{}", path_clone.display(), idx).into()))
                        .px_2()
                        .py_0p5()
                        .rounded_sm()
                        .cursor_pointer()
                        .hover(|style| style.bg(colors_hover))
                        .on_click(fix_click)
                        .child(
                            div()
                                .text_xs()
                                .text_color(colors_accent)
                                .child("Fix"),
                        ),
                )
        }))
}
