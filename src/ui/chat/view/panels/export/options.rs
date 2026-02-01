//! Export panel options component

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;
use crate::ui::chat::view::core::ChatView;

impl ChatView {
    pub(super) fn render_export_options(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let include_meta = self.export.include_metadata;
        let include_tools = self.export.include_tools;
        let include_thinking = self.export.include_thinking;

        let accent = theme.colors.accent;
        let border = theme.colors.border;
        let background = theme.colors.background;
        let surface = theme.colors.surface;
        let text = theme.colors.text;

        let on_toggle_meta = cx.listener(|this, _, _window, cx| {
            this.toggle_export_metadata(cx);
        });

        let on_toggle_tools = cx.listener(|this, _, _window, cx| {
            this.toggle_export_tools(cx);
        });

        let on_toggle_thinking = cx.listener(|this, _, _window, cx| {
            this.toggle_export_thinking(cx);
        });

        div()
            .px_4()
            .py_3()
            .border_t_1()
            .border_color(theme.colors.border)
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .mb_2()
                    .child("Include in Export")
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    // Metadata option
                    .child(
                        div()
                            .id("export-option-metadata")
                            .flex()
                            .items_center()
                            .gap_2()
                            .cursor_pointer()
                            .on_click(on_toggle_meta)
                            .child(
                                div()
                                    .w(px(16.0))
                                    .h(px(16.0))
                                    .rounded_sm()
                                    .border_1()
                                    .border_color(if include_meta { accent } else { border })
                                    .bg(if include_meta { accent } else { background })
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .when(include_meta, |d| {
                                        d.child(div().text_xs().text_color(surface).child("✓"))
                                    })
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(text)
                                    .child("Metadata & Stats")
                            )
                    )
                    // Tools option
                    .child(
                        div()
                            .id("export-option-tools")
                            .flex()
                            .items_center()
                            .gap_2()
                            .cursor_pointer()
                            .on_click(on_toggle_tools)
                            .child(
                                div()
                                    .w(px(16.0))
                                    .h(px(16.0))
                                    .rounded_sm()
                                    .border_1()
                                    .border_color(if include_tools { accent } else { border })
                                    .bg(if include_tools { accent } else { background })
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .when(include_tools, |d| {
                                        d.child(div().text_xs().text_color(surface).child("✓"))
                                    })
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(text)
                                    .child("Tool Calls & Results")
                            )
                    )
                    // Thinking option
                    .child(
                        div()
                            .id("export-option-thinking")
                            .flex()
                            .items_center()
                            .gap_2()
                            .cursor_pointer()
                            .on_click(on_toggle_thinking)
                            .child(
                                div()
                                    .w(px(16.0))
                                    .h(px(16.0))
                                    .rounded_sm()
                                    .border_1()
                                    .border_color(if include_thinking { accent } else { border })
                                    .bg(if include_thinking { accent } else { background })
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .when(include_thinking, |d| {
                                        d.child(div().text_xs().text_color(surface).child("✓"))
                                    })
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(text)
                                    .child("Thinking/Reasoning")
                            )
                    )
            )
    }
}
