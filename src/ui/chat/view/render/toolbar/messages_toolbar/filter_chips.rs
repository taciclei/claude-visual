//! Filter chips render functions

use gpui::*;
use gpui::prelude::*;
use crate::ui::chat::view::core::ChatView;
use crate::ui::chat::view::types::MessageFilter;

impl ChatView {
    /// Renders filter chips with counts and bookmarked filter
    pub fn render_filter_chips(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let current_filter = self.message_filter;
        let visible_count = self.visible_message_count(cx);
        let total_count = self.messages.len();
        let show_bookmarked = self.show_bookmarked_only;
        let bookmarked_count = self.bookmarked_message_count(cx);

        // Pre-compute filter counts for chips
        let filter_counts: Vec<(MessageFilter, usize)> = MessageFilter::all_options()
            .iter()
            .map(|f| (*f, self.message_count_for_filter(*f)))
            .collect();

        div()
            .flex()
            .items_center()
            .gap_1()
            // Filter chips
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .children(filter_counts.iter().map(|(filter, count)| {
                        let is_active = current_filter == *filter;
                        let filter_copy = *filter;
                        let count = *count;
                        div()
                            .id(ElementId::Name(format!("filter-{:?}", filter).into()))
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .py(px(2.0))
                            .rounded_md()
                            .cursor_pointer()
                            .text_xs()
                            .bg(if is_active {
                                theme.colors.accent.opacity(0.2)
                            } else {
                                gpui::transparent_black()
                            })
                            .text_color(if is_active {
                                theme.colors.accent
                            } else {
                                theme.colors.text_muted
                            })
                            .hover(|s| s.bg(theme.colors.surface_hover))
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                this.set_message_filter(filter_copy, cx);
                            }))
                            // Filter icon
                            .child(
                                div()
                                    .text_xs()
                                    .child(filter.icon())
                            )
                            .child(filter.label())
                            // Show count for non-All filters when there are messages
                            .when(*filter != MessageFilter::All && count > 0, |d| {
                                d.child(
                                    div()
                                        .text_color(if is_active {
                                            theme.colors.accent.opacity(0.7)
                                        } else {
                                            theme.colors.text_muted.opacity(0.6)
                                        })
                                        .child(format!("{}", count))
                                )
                            })
                    }))
            )
            // Bookmarked filter toggle
            .when(bookmarked_count > 0, |d| {
                d.child(
                    div()
                        .id("filter-bookmarked")
                        .flex()
                        .items_center()
                        .gap_1()
                        .px_2()
                        .py(px(2.0))
                        .rounded_md()
                        .cursor_pointer()
                        .text_xs()
                        .bg(if show_bookmarked {
                            theme.colors.warning.opacity(0.2)
                        } else {
                            gpui::transparent_black()
                        })
                        .text_color(if show_bookmarked {
                            theme.colors.warning
                        } else {
                            theme.colors.text_muted
                        })
                        .hover(|s| s.bg(theme.colors.warning.opacity(0.1)))
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.toggle_bookmarked_filter(cx);
                        }))
                        .child("★")
                        .child(format!("{}", bookmarked_count))
                )
            })
            // Filter count (when filtered)
            .when(current_filter != MessageFilter::All || show_bookmarked, |d| {
                d.child(
                    div()
                        .text_xs()
                        .text_color(theme.colors.text_muted)
                        .child(format!("({}/{})", visible_count, total_count))
                )
            })
    }
}
