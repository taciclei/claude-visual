//! Main render implementation for ActivityPanel

use gpui::*;
use gpui::prelude::*;

use super::{ActivityPanel, ActivityPanelEvent};

impl ActivityPanel {
    /// Render empty state
    pub(super) fn render_empty_state(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let text_muted = theme.colors.text_muted;

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .py_8()
            .child(
                div()
                    .text_2xl()
                    .text_color(text_muted)
                    .child("📋"),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(text_muted)
                    .mt_2()
                    .child("No activity yet"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(text_muted)
                    .mt_1()
                    .child("Team activity will appear here"),
            )
    }

    /// Render loading state
    pub(super) fn render_loading(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let text_muted = theme.colors.text_muted;

        div()
            .flex()
            .items_center()
            .justify_center()
            .py_8()
            .child(
                div()
                    .text_sm()
                    .text_color(text_muted)
                    .child("Loading activity..."),
            )
    }
}

impl Render for ActivityPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let filtered = self.filtered_activities();

        let background = theme.colors.background;
        let border_color = theme.colors.border;
        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;
        let surface_hover = theme.colors.surface_hover;

        let refresh_listener = cx.listener(|_this, _, _window, cx| {
            cx.emit(ActivityPanelEvent::Refresh);
        });

        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .flex()
            .flex_col()
            .bg(background)
            .child(
                // Header
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .py_3()
                    .border_b_1()
                    .border_color(border_color)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(text_color)
                                    .child("Activity Feed"),
                            )
                            .when(self.team_name.is_some(), |d| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .text_color(text_muted)
                                        .child(self.team_name.clone().unwrap_or_default()),
                                )
                            }),
                    )
                    .child(
                        div()
                            .id("refresh-activity")
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .hover(move |s| s.bg(surface_hover))
                            .cursor_pointer()
                            .text_sm()
                            .text_color(text_muted)
                            .on_click(refresh_listener)
                            .child("↻"),
                    ),
            )
            // Filters
            .child(self.render_filters(cx))
            // Content
            .child(
                div()
                    .flex_1()
                    .id("scroll-activity-list")
                    .overflow_y_scroll()
                    .child(if self.is_loading {
                        self.render_loading(cx).into_any_element()
                    } else if filtered.is_empty() {
                        self.render_empty_state(cx).into_any_element()
                    } else {
                        div()
                            .flex()
                            .flex_col()
                            .children(filtered.into_iter().map(|entry| {
                                self.render_activity_entry(entry, cx)
                            }))
                            .into_any_element()
                    }),
            )
    }
}
