//! Syntax tab rendering

use gpui::*;

use crate::ui::extensions::theme_editor::ThemeEditor;
use crate::ui::extensions::theme_editor::types::EditingColor;

impl ThemeEditor {
    /// Render syntax tab
    pub(crate) fn render_syntax_tab(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .flex()
            .flex_col()
            .gap_4()
            // Syntax colors
            .child(self.render_color_section("Syntax Highlighting", EditingColor::all_syntax_colors(), theme, cx))
            // Preview
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
                            .child("Preview"),
                    )
                    .child(self.render_code_preview(cx)),
            )
    }
}
