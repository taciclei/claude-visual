//! Skeleton component for card/container layouts

use std::sync::Arc;
use gpui::*;
use gpui::prelude::*;
use crate::app::state::AppState;
use crate::ui::pct;

/// Skeleton for a card/container
pub struct SkeletonCard {
    app_state: Arc<AppState>,
    /// Show avatar placeholder
    show_avatar: bool,
    /// Number of text lines
    text_lines: usize,
    /// Show action buttons
    show_actions: bool,
}

impl SkeletonCard {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            show_avatar: true,
            text_lines: 2,
            show_actions: false,
        }
    }

    pub fn set_show_avatar(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_avatar = show;
        cx.notify();
    }

    pub fn set_text_lines(&mut self, lines: usize, cx: &mut Context<Self>) {
        self.text_lines = lines;
        cx.notify();
    }

    pub fn set_show_actions(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_actions = show;
        cx.notify();
    }
}

impl Render for SkeletonCard {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let base_color = theme.colors.surface_hover;

        div()
            .id("skeleton-card")
            .p_4()
            .rounded_lg()
            .border_1()
            .border_color(theme.colors.border)
            .bg(theme.colors.surface)
            .flex()
            .flex_col()
            .gap_3()
            // Header row (avatar + title)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    // Avatar
                    .when(self.show_avatar, |d| {
                        d.child(
                            div()
                                .size(px(40.0))
                                .rounded_full()
                                .bg(base_color)
                                .flex_shrink_0()
                        )
                    })
                    // Title lines
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .h(px(14.0))
                                    .w(pct(60.0))
                                    .rounded(px(2.0))
                                    .bg(base_color)
                            )
                            .child(
                                div()
                                    .h(px(10.0))
                                    .w(pct(40.0))
                                    .rounded(px(2.0))
                                    .bg(base_color)
                            )
                    )
            )
            // Text lines
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .children((0..self.text_lines).map(|i| {
                        let is_last = i == self.text_lines - 1;
                        let width = if is_last { 70.0 } else { 100.0 };

                        div()
                            .h(px(12.0))
                            .w(pct(width))
                            .rounded(px(2.0))
                            .bg(base_color)
                    }))
            )
            // Actions
            .when(self.show_actions, |d| {
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .mt_2()
                        .child(
                            div()
                                .h(px(32.0))
                                .w(px(80.0))
                                .rounded(px(4.0))
                                .bg(base_color)
                        )
                        .child(
                            div()
                                .h(px(32.0))
                                .w(px(80.0))
                                .rounded(px(4.0))
                                .bg(base_color)
                        )
                )
            })
    }
}
