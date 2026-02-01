//! Main timeline component

use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;

use crate::app::state::AppState;

use super::types::*;

/// Timeline component
pub struct Timeline {
    app_state: Arc<AppState>,
    /// Timeline items
    pub(crate) items: Vec<TimelineItem>,
    /// Orientation
    pub(crate) orientation: TimelineOrientation,
    /// Show connector lines
    pub(crate) show_connectors: bool,
    /// Alternate sides (for vertical)
    pub(crate) alternate: bool,
}

impl Timeline {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self {
            app_state,
            items: Vec::new(),
            orientation: TimelineOrientation::default(),
            show_connectors: true,
            alternate: false,
        }
    }

    /// Set items
    pub fn set_items(&mut self, items: Vec<TimelineItem>, cx: &mut Context<Self>) {
        self.items = items;
        cx.notify();
    }

    /// Add item
    pub fn add_item(&mut self, item: TimelineItem, cx: &mut Context<Self>) {
        self.items.push(item);
        cx.notify();
    }

    /// Set orientation
    pub fn set_orientation(&mut self, orientation: TimelineOrientation, cx: &mut Context<Self>) {
        self.orientation = orientation;
        cx.notify();
    }

    /// Set show connectors
    pub fn set_show_connectors(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_connectors = show;
        cx.notify();
    }

    /// Set alternate
    pub fn set_alternate(&mut self, alternate: bool, cx: &mut Context<Self>) {
        self.alternate = alternate;
        cx.notify();
    }
}

impl EventEmitter<TimelineEvent> for Timeline {}

impl Render for Timeline {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let is_vertical = matches!(self.orientation, TimelineOrientation::Vertical);

        div()
            .id("timeline")
            .w_full()
            .flex()
            .when(is_vertical, |d| d.flex_col())
            .when(!is_vertical, |d| d.flex_row().items_start())
            .children(self.items.iter().enumerate().map(|(index, item)| {
                let is_last = index == self.items.len() - 1;
                let is_left = self.alternate && index % 2 == 0;

                let status_color = match item.status {
                    TimelineItemStatus::Pending => theme.colors.text_muted,
                    TimelineItemStatus::InProgress => theme.colors.accent,
                    TimelineItemStatus::Completed => hsla(0.38, 0.7, 0.45, 1.0), // Green
                    TimelineItemStatus::Error => theme.colors.error,
                };

                let icon = item
                    .icon
                    .clone()
                    .unwrap_or_else(|| item.status.icon().to_string());

                if is_vertical {
                    div()
                        .id(SharedString::from(format!("timeline-item-{}", index)))
                        .w_full()
                        .flex()
                        .when(is_left && self.alternate, |d| d.flex_row_reverse())
                        // Dot/icon column
                        .child(
                            div()
                                .w(px(40.0))
                                .flex()
                                .flex_col()
                                .items_center()
                                // Dot
                                .child(
                                    div()
                                        .size(px(24.0))
                                        .rounded_full()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .bg(status_color.opacity(0.15))
                                        .text_color(status_color)
                                        .text_sm()
                                        .child(icon),
                                )
                                // Connector line
                                .when(self.show_connectors && !is_last, |d| {
                                    d.child(
                                        div()
                                            .w(px(2.0))
                                            .flex_1()
                                            .min_h(px(24.0))
                                            .bg(theme.colors.border),
                                    )
                                }),
                        )
                        // Content column
                        .child(
                            div()
                                .flex_1()
                                .pb_4()
                                .flex()
                                .flex_col()
                                .gap_1()
                                // Timestamp
                                .when_some(item.timestamp.clone(), |d, ts| {
                                    d.child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child(ts),
                                    )
                                })
                                // Title
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.colors.text)
                                        .child(item.title.clone()),
                                )
                                // Description
                                .when_some(item.description.clone(), |d, desc| {
                                    d.child(
                                        div()
                                            .text_sm()
                                            .text_color(theme.colors.text_muted)
                                            .child(desc),
                                    )
                                }),
                        )
                        .into_any_element()
                } else {
                    // Horizontal layout
                    div()
                        .id(SharedString::from(format!("timeline-item-{}", index)))
                        .flex()
                        .flex_col()
                        .items_center()
                        .flex_1()
                        // Dot row
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .w_full()
                                // Left connector
                                .when(self.show_connectors && index > 0, |d| {
                                    d.child(div().flex_1().h(px(2.0)).bg(theme.colors.border))
                                })
                                // Dot
                                .child(
                                    div()
                                        .size(px(24.0))
                                        .rounded_full()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .bg(status_color.opacity(0.15))
                                        .text_color(status_color)
                                        .text_sm()
                                        .flex_shrink_0()
                                        .child(icon),
                                )
                                // Right connector
                                .when(self.show_connectors && !is_last, |d| {
                                    d.child(div().flex_1().h(px(2.0)).bg(theme.colors.border))
                                }),
                        )
                        // Content
                        .child(
                            div()
                                .mt_2()
                                .text_center()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.colors.text)
                                        .child(item.title.clone()),
                                )
                                .when_some(item.timestamp.clone(), |d, ts| {
                                    d.child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child(ts),
                                    )
                                }),
                        )
                        .into_any_element()
                }
            }))
    }
}
