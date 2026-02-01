//! Empty state rendering for diagnostics panel

use gpui::prelude::*;
use gpui::*;

use crate::ui::lsp::diagnostics::core::DiagnosticsPanel;
use crate::ui::lsp::diagnostics::types::{DiagnosticsPanelEvent, SimpleColors};

pub fn render_empty_state(
    colors: &SimpleColors,
    cx: &mut Context<DiagnosticsPanel>,
) -> impl IntoElement {
    let text_muted = colors.text_muted;
    let text = colors.text;
    let success = colors.success;
    let accent = colors.accent;

    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .py_6()
        .gap_3()
        .child(
            div()
                .size(px(40.0))
                .rounded_full()
                .bg(success.opacity(0.1))
                .flex()
                .items_center()
                .justify_center()
                .child(div().text_lg().text_color(success).child("✓")),
        )
        .child(
            div()
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .text_color(text)
                .child("No problems detected"),
        )
        .child(
            div()
                .text_xs()
                .text_color(text_muted)
                .child("Your code is looking good!"),
        )
        // Quick skill suggestions
        .child(
            div()
                .pt_2()
                .flex()
                .flex_wrap()
                .justify_center()
                .gap_2()
                // Review code skill
                .child(
                    div()
                        .id("diagnostics-empty-review")
                        .px_2()
                        .py_1()
                        .rounded_md()
                        .cursor_pointer()
                        .bg(accent.opacity(0.15))
                        .border_1()
                        .border_color(accent.opacity(0.3))
                        .text_xs()
                        .text_color(accent)
                        .hover(move |s| {
                            s.bg(accent.opacity(0.25)).border_color(accent.opacity(0.5))
                        })
                        .on_click(cx.listener(|_this, _, _window, cx| {
                            cx.emit(DiagnosticsPanelEvent::SendSkillCommand(
                                "/review".to_string(),
                            ));
                        }))
                        .child("🔍 /review"),
                )
                // Auto-fix skill
                .child(
                    div()
                        .id("diagnostics-empty-autofix")
                        .px_2()
                        .py_1()
                        .rounded_md()
                        .cursor_pointer()
                        .bg(success.opacity(0.15))
                        .border_1()
                        .border_color(success.opacity(0.3))
                        .text_xs()
                        .text_color(success)
                        .hover(move |s| {
                            s.bg(success.opacity(0.25))
                                .border_color(success.opacity(0.5))
                        })
                        .on_click(cx.listener(|_this, _, _window, cx| {
                            cx.emit(DiagnosticsPanelEvent::SendSkillCommand(
                                "/utils:auto-fix".to_string(),
                            ));
                        }))
                        .child("🔧 /auto-fix"),
                ),
        )
}
