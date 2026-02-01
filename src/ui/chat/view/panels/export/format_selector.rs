//! Export panel format selector component

use gpui::prelude::*;
use gpui::*;

use crate::app::theme::Theme;
use crate::ui::chat::view::core::ChatView;
use crate::ui::chat::view::types::ExportFormat;

impl ChatView {
    pub(super) fn render_export_format_selector(
        &self,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let current_format = self.export.format;
        let text_color = theme.colors.text;
        let accent_color = theme.colors.accent;

        div()
            .px_4()
            .py_3()
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .mb_2()
                    .child("Export Format"),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .children(ExportFormat::all().iter().map(|format| {
                        let is_selected = *format == current_format;
                        let format_copy = *format;
                        let on_select = cx.listener(move |this, _, _window, cx| {
                            this.set_export_format(format_copy, cx);
                        });

                        div()
                            .id(ElementId::Name(format!("format-{:?}", format).into()))
                            .flex_1()
                            .px_3()
                            .py_2()
                            .rounded_lg()
                            .cursor_pointer()
                            .border_1()
                            .when(is_selected, |d| {
                                d.bg(theme.colors.accent.opacity(0.15))
                                    .border_color(theme.colors.accent)
                            })
                            .when(!is_selected, |d| {
                                d.bg(theme.colors.background)
                                    .border_color(theme.colors.border)
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                            })
                            .on_click(on_select)
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .gap_1()
                                    .child(div().text_base().child(format.icon()))
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(if is_selected {
                                                FontWeight::SEMIBOLD
                                            } else {
                                                FontWeight::NORMAL
                                            })
                                            .text_color(if is_selected {
                                                accent_color
                                            } else {
                                                text_color
                                            })
                                            .child(format.display_name()),
                                    ),
                            )
                    })),
            )
    }
}
