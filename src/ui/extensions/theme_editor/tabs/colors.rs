//! Colors tab rendering

use gpui::*;

use crate::app::theme::ThemeVariant;
use crate::ui::extensions::theme_editor::types::EditingColor;
use crate::ui::extensions::theme_editor::ThemeEditor;

impl ThemeEditor {
    /// Render colors tab
    pub(crate) fn render_colors_tab(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .flex()
            .flex_col()
            .gap_4()
            // Variant selector
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
                            .child("Theme Variant"),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(self.render_variant_button(ThemeVariant::Dark, theme, cx))
                            .child(self.render_variant_button(ThemeVariant::Light, theme, cx)),
                    ),
            )
            // Color sections
            .child(self.render_color_section(
                "Background & Surface",
                &[
                    EditingColor::Background,
                    EditingColor::Surface,
                    EditingColor::SurfaceHover,
                    EditingColor::Border,
                ],
                theme,
                cx,
            ))
            .child(self.render_color_section(
                "Text",
                &[EditingColor::Text, EditingColor::TextMuted],
                theme,
                cx,
            ))
            .child(self.render_color_section(
                "Accent",
                &[EditingColor::Accent, EditingColor::AccentHover],
                theme,
                cx,
            ))
            .child(self.render_color_section(
                "Status",
                &[
                    EditingColor::Success,
                    EditingColor::Warning,
                    EditingColor::Error,
                    EditingColor::Info,
                ],
                theme,
                cx,
            ))
            .child(self.render_color_section(
                "Interaction",
                &[EditingColor::FocusRing, EditingColor::Selection],
                theme,
                cx,
            ))
    }
}
