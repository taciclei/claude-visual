//! Emoji picker rendering

use gpui::prelude::*;
use gpui::*;

use super::emoji_picker::EmojiPicker;
use super::types::*;

impl RenderOnce for EmojiPicker {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let width = self.size.width();
        let emoji_size = self.size.emoji_size();
        let grid_cols = self.size.grid_cols();

        let filtered_emojis: Vec<_> = if !self.search_query.is_empty() {
            let q = self.search_query.to_lowercase();
            self.emojis
                .iter()
                .filter(|e| e.name.to_lowercase().contains(&q))
                .collect()
        } else if self.selected_category == EmojiCategory::Recent {
            self.emojis
                .iter()
                .filter(|e| self.recent.contains(&e.emoji))
                .collect()
        } else {
            self.emojis
                .iter()
                .filter(|e| e.category == self.selected_category)
                .collect()
        };

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .w(px(width))
            .bg(hsla(0.0, 0.0, 0.12, 1.0))
            .border_1()
            .border_color(hsla(0.0, 0.0, 0.2, 1.0))
            .rounded(px(8.0))
            .shadow_lg()
            // Search
            .when(self.show_search, |el| {
                el.child(
                    div()
                        .p(px(8.0))
                        .border_b_1()
                        .border_color(hsla(0.0, 0.0, 0.2, 1.0))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .px(px(10.0))
                                .py(px(6.0))
                                .bg(hsla(0.0, 0.0, 0.08, 1.0))
                                .rounded(px(6.0))
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                                        .child("🔍"),
                                )
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(if self.search_query.is_empty() {
                                            hsla(0.0, 0.0, 0.5, 1.0)
                                        } else {
                                            hsla(0.0, 0.0, 0.9, 1.0)
                                        })
                                        .child(if self.search_query.is_empty() {
                                            "Search emoji...".to_string()
                                        } else {
                                            self.search_query.to_string()
                                        }),
                                ),
                        ),
                )
            })
            // Category tabs
            .when(self.show_categories, |el| {
                el.child(
                    div()
                        .flex()
                        .gap(px(2.0))
                        .px(px(4.0))
                        .py(px(4.0))
                        .border_b_1()
                        .border_color(hsla(0.0, 0.0, 0.2, 1.0))
                        .children(EmojiCategory::all().into_iter().map(|cat| {
                            let is_selected = cat == self.selected_category;
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .w(px(28.0))
                                .h(px(28.0))
                                .rounded(px(4.0))
                                .bg(if is_selected {
                                    hsla(0.6, 0.5, 0.4, 0.2)
                                } else {
                                    hsla(0.0, 0.0, 0.0, 0.0)
                                })
                                .cursor_pointer()
                                .child(
                                    div()
                                        .text_size(px(16.0))
                                        .opacity(if is_selected { 1.0 } else { 0.6 })
                                        .child(cat.icon()),
                                )
                        })),
                )
            })
            // Category label
            .child(
                div()
                    .px(px(12.0))
                    .py(px(8.0))
                    .text_size(px(11.0))
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                    .child(self.selected_category.label().to_uppercase()),
            )
            // Emoji grid
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .px(px(8.0))
                    .pb(px(8.0))
                    .max_h(px(200.0))
                    .id("scroll-emoji-grid")
                    .overflow_y_scroll()
                    .when(filtered_emojis.is_empty(), |el| {
                        el.child(
                            div()
                                .w_full()
                                .p(px(16.0))
                                .text_size(px(13.0))
                                .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                                .child("No emoji found"),
                        )
                    })
                    .children(filtered_emojis.iter().take(grid_cols * 6).map(|emoji| {
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(emoji_size))
                            .h(px(emoji_size))
                            .rounded(px(4.0))
                            .cursor_pointer()
                            .child(
                                div()
                                    .text_size(px(emoji_size * 0.7))
                                    .child(emoji.emoji.clone()),
                            )
                    })),
            )
            // Preview
            .when(self.show_preview, |el| {
                el.child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .px(px(12.0))
                        .py(px(8.0))
                        .border_t_1()
                        .border_color(hsla(0.0, 0.0, 0.2, 1.0))
                        .child(div().text_size(px(24.0)).child("😀"))
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(hsla(0.0, 0.0, 0.6, 1.0))
                                .child(":grinning:"),
                        ),
                )
            })
    }
}
