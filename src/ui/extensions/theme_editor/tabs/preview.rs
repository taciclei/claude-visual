//! Preview tab rendering

use crate::ui::span;
use gpui::*;

use crate::ui::extensions::theme_editor::ThemeEditor;

impl ThemeEditor {
    /// Render preview tab
    pub(crate) fn render_preview_tab(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let editing = &self.editing_theme;

        div()
            .flex()
            .flex_col()
            .gap_4()
            // UI Preview
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.colors.text)
                            .child("UI Preview"),
                    )
                    .child(
                        div()
                            .p_4()
                            .rounded_lg()
                            .bg(editing.colors.background)
                            .border_1()
                            .border_color(editing.colors.border)
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_3()
                                    // Header
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .text_base()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(editing.colors.text)
                                                    .child("Theme Preview"),
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(editing.colors.text_muted)
                                                    .child("Subtitle text"),
                                            ),
                                    )
                                    // Surface card
                                    .child(
                                        div()
                                            .p_3()
                                            .rounded_md()
                                            .bg(editing.colors.surface)
                                            .border_1()
                                            .border_color(editing.colors.border)
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(editing.colors.text)
                                                    .child("Surface content area"),
                                            ),
                                    )
                                    // Buttons
                                    .child(
                                        div()
                                            .flex()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .px_3()
                                                    .py_1()
                                                    .rounded_md()
                                                    .bg(editing.colors.accent)
                                                    .text_sm()
                                                    .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                                                    .child("Primary"),
                                            )
                                            .child(
                                                div()
                                                    .px_3()
                                                    .py_1()
                                                    .rounded_md()
                                                    .bg(editing.colors.surface_hover)
                                                    .text_sm()
                                                    .text_color(editing.colors.text)
                                                    .child("Secondary"),
                                            ),
                                    )
                                    // Status colors
                                    .child(
                                        div()
                                            .flex()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .size(px(12.0))
                                                    .rounded_full()
                                                    .bg(editing.colors.success),
                                            )
                                            .child(
                                                div()
                                                    .size(px(12.0))
                                                    .rounded_full()
                                                    .bg(editing.colors.warning),
                                            )
                                            .child(
                                                div()
                                                    .size(px(12.0))
                                                    .rounded_full()
                                                    .bg(editing.colors.error),
                                            )
                                            .child(
                                                div()
                                                    .size(px(12.0))
                                                    .rounded_full()
                                                    .bg(editing.colors.info),
                                            ),
                                    ),
                            ),
                    ),
            )
            // Code Preview
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.colors.text)
                            .child("Code Preview"),
                    )
                    .child(self.render_code_preview(cx)),
            )
            // Apply preview button
            .child(
                div()
                    .id("apply-preview")
                    .px_4()
                    .py_2()
                    .rounded_md()
                    .bg(theme.colors.accent)
                    .text_sm()
                    .text_color(hsla(0.0, 0.0, 1.0, 1.0))
                    .cursor_pointer()
                    .hover(|s| s.opacity(0.9))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.apply_preview(cx);
                    }))
                    .child("Apply Preview to App"),
            )
    }

    /// Render code preview
    pub(crate) fn render_code_preview(&self, _cx: &Context<Self>) -> impl IntoElement {
        let editing = &self.editing_theme;

        div()
            .p_3()
            .rounded_md()
            .bg(editing.colors.background)
            .border_1()
            .border_color(editing.colors.border)
            .font_family("JetBrains Mono")
            .text_sm()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_px()
                    // Line 1: function definition
                    .child(
                        div()
                            .flex()
                            .child(span().text_color(editing.syntax.keyword).child("fn "))
                            .child(
                                span()
                                    .text_color(editing.syntax.function)
                                    .child("calculate"),
                            )
                            .child(span().text_color(editing.syntax.punctuation).child("("))
                            .child(span().text_color(editing.syntax.variable).child("x"))
                            .child(span().text_color(editing.syntax.punctuation).child(": "))
                            .child(span().text_color(editing.syntax.type_name).child("i32"))
                            .child(span().text_color(editing.syntax.punctuation).child(") -> "))
                            .child(span().text_color(editing.syntax.type_name).child("i32"))
                            .child(span().text_color(editing.syntax.punctuation).child(" {")),
                    )
                    // Line 2: comment
                    .child(
                        div()
                            .pl_4()
                            .text_color(editing.syntax.comment)
                            .child("// Calculate result"),
                    )
                    // Line 3: let binding
                    .child(
                        div()
                            .pl_4()
                            .flex()
                            .child(span().text_color(editing.syntax.keyword).child("let "))
                            .child(span().text_color(editing.syntax.variable).child("result "))
                            .child(span().text_color(editing.syntax.operator).child("= "))
                            .child(span().text_color(editing.syntax.variable).child("x "))
                            .child(span().text_color(editing.syntax.operator).child("* "))
                            .child(span().text_color(editing.syntax.number).child("2"))
                            .child(span().text_color(editing.syntax.punctuation).child(";")),
                    )
                    // Line 4: string
                    .child(
                        div()
                            .pl_4()
                            .flex()
                            .child(span().text_color(editing.syntax.function).child("println!"))
                            .child(span().text_color(editing.syntax.punctuation).child("("))
                            .child(
                                span()
                                    .text_color(editing.syntax.string)
                                    .child("\"Result: {}\""),
                            )
                            .child(span().text_color(editing.syntax.punctuation).child(", "))
                            .child(span().text_color(editing.syntax.variable).child("result"))
                            .child(span().text_color(editing.syntax.punctuation).child(");")),
                    )
                    // Line 5: return
                    .child(
                        div()
                            .pl_4()
                            .flex()
                            .child(span().text_color(editing.syntax.variable).child("result")),
                    )
                    // Line 6: closing brace
                    .child(div().text_color(editing.syntax.punctuation).child("}")),
            )
    }
}
