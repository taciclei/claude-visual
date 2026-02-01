//! Search bar rendering

use gpui::*;
use gpui::prelude::*;

use super::view::CodeBlockView;

impl CodeBlockView {
    /// Render the search bar
    pub(crate) fn render_search_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();
        let search_query = self.search_query.clone();
        let match_count = self.search_matches.len();
        let current_match = self.current_match_index;

        div()
            .flex()
            .items_center()
            .gap_2()
            .px_3()
            .py_2()
            .bg(theme.colors.surface)
            .border_b_1()
            .border_color(theme.colors.border)
            .child(
                div()
                    .flex_1()
                    .px_2()
                    .py_1()
                    .bg(theme.colors.background)
                    .rounded_sm()
                    .border_1()
                    .border_color(theme.colors.border)
                    .text_xs()
                    .text_color(theme.colors.text)
                    .child(if search_query.is_empty() {
                        "Type to search...".to_string()
                    } else {
                        search_query.clone()
                    })
            )
            .child(self.render_search_controls(match_count, current_match, &search_query, &theme, cx))
    }

    /// Render search control buttons
    fn render_search_controls(
        &self,
        match_count: usize,
        current_match: Option<usize>,
        search_query: &str,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_1()
            // Match count
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .child(if match_count > 0 {
                        format!(
                            "{}/{}",
                            current_match.map(|i| i + 1).unwrap_or(0),
                            match_count
                        )
                    } else if !search_query.is_empty() {
                        "No matches".to_string()
                    } else {
                        String::new()
                    })
            )
            // Previous match
            .child(
                div()
                    .id("search-prev-button")
                    .px_1()
                    .py_0p5()
                    .rounded_sm()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .cursor_pointer()
                    .hover(|s| s.bg(theme.colors.surface_hover))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.prev_match(cx);
                    }))
                    .child("↑")
            )
            // Next match
            .child(
                div()
                    .id("search-next-button")
                    .px_1()
                    .py_0p5()
                    .rounded_sm()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .cursor_pointer()
                    .hover(|s| s.bg(theme.colors.surface_hover))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.next_match(cx);
                    }))
                    .child("↓")
            )
            // Close search
            .child(
                div()
                    .id("search-close-button")
                    .px_1()
                    .py_0p5()
                    .rounded_sm()
                    .text_xs()
                    .text_color(theme.colors.text_muted)
                    .cursor_pointer()
                    .hover(|s| s.bg(theme.colors.surface_hover))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.hide_search(cx);
                    }))
                    .child("✕")
            )
    }
}
