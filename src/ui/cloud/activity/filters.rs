//! Activity filter UI components

use gpui::*;

use crate::cloud::team::{ActivityEntry, ActivityType};

use super::ActivityPanel;

impl ActivityPanel {
    /// Filter activities
    pub(super) fn filtered_activities(&self) -> Vec<&ActivityEntry> {
        self.activities
            .iter()
            .filter(|a| {
                if let Some(ref filter_type) = self.filter_type {
                    if &a.activity_type != filter_type {
                        return false;
                    }
                }
                if let Some(ref filter_user) = self.filter_user {
                    if &a.user_id != filter_user {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    /// Set filter type
    pub(super) fn set_filter_type(&mut self, filter: Option<ActivityType>, cx: &mut Context<Self>) {
        self.filter_type = filter;
        cx.notify();
    }

    /// Render filter buttons
    pub(super) fn render_filters(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let border_color = theme.colors.border;

        div()
            .flex()
            .gap_1()
            .px_4()
            .py_2()
            .border_b_1()
            .border_color(border_color)
            .id("scroll-activity-filters")
            .overflow_x_scroll()
            .child(self.render_filter_button("All", None, cx))
            .child(self.render_filter_button("Created", Some(ActivityType::Created), cx))
            .child(self.render_filter_button("Updated", Some(ActivityType::Updated), cx))
            .child(self.render_filter_button("Shared", Some(ActivityType::Shared), cx))
            .child(self.render_filter_button("Members", Some(ActivityType::Joined), cx))
    }

    /// Render filter button
    fn render_filter_button(
        &self,
        label: &str,
        filter: Option<ActivityType>,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let is_active = self.filter_type == filter;
        let label = label.to_string();

        let bg_normal = theme.colors.surface;
        let bg_active = theme.colors.accent;
        let bg_hover_normal = theme.colors.surface_hover;
        let bg_hover_active = theme.colors.accent_hover;
        let text_color = theme.colors.text;

        let on_click = cx.listener(move |this, _, _window, cx| {
            this.set_filter_type(filter, cx);
        });

        div()
            .id(ElementId::Name(format!("filter-{}", label).into()))
            .px_2()
            .py_1()
            .rounded_md()
            .bg(if is_active { bg_active } else { bg_normal })
            .hover(move |s| {
                s.bg(if is_active {
                    bg_hover_active
                } else {
                    bg_hover_normal
                })
            })
            .cursor_pointer()
            .text_xs()
            .text_color(text_color)
            .on_click(on_click)
            .child(label)
    }
}
