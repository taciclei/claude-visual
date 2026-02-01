//! Templates panel search bar

use gpui::*;
use gpui::prelude::*;

use super::super::super::core::ChatView;

impl ChatView {
    pub fn render_templates_search(
        &self,
        theme: &crate::app::theme::Theme,
        filter: String,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;
        let border_color = theme.colors.border;
        let background = theme.colors.background;

        div()
            .px_4()
            .py_2()
            .border_b_1()
            .border_color(border_color)
            .child(
                div()
                    .w_full()
                    .h(px(32.0))
                    .px_3()
                    .rounded_md()
                    .bg(background)
                    .border_1()
                    .border_color(border_color)
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .text_color(text_muted)
                            .child("🔍")
                    )
                    .child(
                        div()
                            .flex_1()
                            .text_sm()
                            .text_color(if filter.is_empty() { text_muted } else { text_color })
                            .child(if filter.is_empty() {
                                "Search templates...".to_string()
                            } else {
                                filter
                            })
                    )
            )
    }
}
