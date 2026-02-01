//! Field rendering for server configuration

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;
use crate::ui::mcp::server_config::core::ServerConfigEditor;
use crate::ui::mcp::server_config::types::EditingField;

impl ServerConfigEditor {
    /// Render a text input field
    pub(crate) fn render_field(
        &self,
        label: &str,
        field: EditingField,
        value: &str,
        placeholder: &str,
        multiline: bool,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let value_clone = value.to_string();
        let label = label.to_string();
        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;
        let surface = theme.colors.surface;
        let accent = theme.colors.accent;
        let border = theme.colors.border;

        div()
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(text_color)
                    .child(label),
            )
            .child(
                div()
                    .w_full()
                    .when(multiline, |d| d.min_h(px(80.0)))
                    .when(!multiline, |d| d.h(px(36.0)))
                    .px_3()
                    .py_2()
                    .bg(surface)
                    .border_1()
                    .border_color(if self.focused_field == Some(field) {
                        accent
                    } else {
                        border
                    })
                    .rounded_md()
                    .child(
                        div()
                            .text_sm()
                            .text_color(if value.is_empty() {
                                text_muted
                            } else {
                                text_color
                            })
                            .when(multiline, |d| d.whitespace_nowrap())
                            .child(if value.is_empty() {
                                placeholder.to_string()
                            } else {
                                value_clone
                            }),
                    ),
            )
    }
}
