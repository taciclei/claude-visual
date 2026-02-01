use gpui::*;
use gpui::prelude::*;
use super::super::SettingsModal;

impl SettingsModal {
    /// Render a settings section
    pub(crate) fn render_section(
        &self,
        title: &str,
        description: &str,
        content: impl IntoElement,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let title = title.to_string();
        let description = description.to_string();

        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.colors.text)
                    .child(title),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child(description),
            )
            .child(
                div()
                    .mt_2()
                    .p_3()
                    .rounded_md()
                    .bg(theme.colors.surface)
                    .child(content),
            )
    }
}
