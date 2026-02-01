//! Activity entry rendering

use gpui::*;
use gpui::prelude::*;

use crate::cloud::team::{ActivityEntry, ActivityType};

use super::ActivityPanel;

impl ActivityPanel {
    /// Navigate to target
    pub(super) fn navigate_to(&self, entry: &ActivityEntry, cx: &mut Context<Self>) {
        cx.emit(super::ActivityPanelEvent::NavigateToTarget {
            target_type: entry.target_type,
            target_id: entry.target_id.clone(),
        });
    }

    /// Format relative time
    pub(super) fn format_time(&self, timestamp: &chrono::DateTime<chrono::Utc>) -> String {
        let now = chrono::Utc::now();
        let diff = now.signed_duration_since(*timestamp);

        if diff.num_seconds() < 60 {
            "just now".to_string()
        } else if diff.num_minutes() < 60 {
            format!("{}m ago", diff.num_minutes())
        } else if diff.num_hours() < 24 {
            format!("{}h ago", diff.num_hours())
        } else if diff.num_days() < 7 {
            format!("{}d ago", diff.num_days())
        } else {
            timestamp.format("%b %d").to_string()
        }
    }

    /// Render activity entry
    pub(super) fn render_activity_entry(&self, entry: &ActivityEntry, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let entry_clone = entry.clone();

        let icon_color = match entry.activity_type {
            ActivityType::Created => theme.colors.success,
            ActivityType::Deleted => theme.colors.error,
            ActivityType::Updated | ActivityType::Shared => theme.colors.accent,
            ActivityType::Joined | ActivityType::Left => theme.colors.warning,
            _ => theme.colors.text_muted,
        };

        let border_color = theme.colors.border.opacity(0.5);
        let surface_hover = theme.colors.surface_hover;
        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;

        let on_click = cx.listener(move |this, _, _window, cx| {
            this.navigate_to(&entry_clone, cx);
        });

        div()
            .id(ElementId::Name(format!("activity-{}", entry.id).into()))
            .flex()
            .gap_3()
            .px_4()
            .py_3()
            .border_b_1()
            .border_color(border_color)
            .hover(move |s| s.bg(surface_hover))
            .cursor_pointer()
            .on_click(on_click)
            .child(
                // Icon
                div()
                    .size_8()
                    .rounded_full()
                    .bg(icon_color.opacity(0.15))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_sm()
                    .text_color(icon_color)
                    .child(entry.activity_type.icon()),
            )
            .child(
                // Content
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(text_color)
                                    .child(entry.user_name.clone().unwrap_or_else(|| "Unknown".to_string())),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(text_muted)
                                    .child(entry.activity_type.verb()),
                            )
                            .when(entry.target_name.is_some(), |d| {
                                d.child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(text_color)
                                        .child(entry.target_name.clone().unwrap_or_default()),
                                )
                            }),
                    )
                    .when(entry.details.is_some(), |d| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(text_muted)
                                .child(entry.details.clone().unwrap_or_default()),
                        )
                    }),
            )
            .child(
                // Timestamp
                div()
                    .text_xs()
                    .text_color(text_muted)
                    .child(self.format_time(&entry.timestamp)),
            )
    }
}
