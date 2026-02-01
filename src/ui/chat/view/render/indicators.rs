//! Indicator render functions for ChatView

use gpui::*;
use gpui::prelude::*;
use crate::ui::pct;
use super::super::core::ChatView;

impl ChatView {
    pub fn render_file_drop_zone(&self, theme: &crate::app::theme::Theme) -> Div {
        div()
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(theme.colors.accent.opacity(0.1))
            .border_4()
            .border_color(theme.colors.accent.opacity(0.5))
            .rounded_xl()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_4()
                    .child(
                        div()
                            .text_size(px(48.0))
                            .child("📂")
                    )
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.colors.accent)
                            .child("Drop files here")
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.colors.text_muted)
                            .child(if self.drag_file_count > 1 {
                                format!("{} files ready to add", self.drag_file_count)
                            } else {
                                "File will be added to context".to_string()
                            })
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .mt_2()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .bg(theme.colors.surface)
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child("📄")
                                    .child("Code")
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .bg(theme.colors.surface)
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child("📝")
                                    .child("Text")
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .bg(theme.colors.surface)
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .child("🖼️")
                                    .child("Image")
                            )
                    )
            )
    }


    pub fn render_session_health_indicator(&self, theme: &crate::app::theme::Theme, _cx: &mut Context<Self>) -> Div {
        let health_color = self.stats.health_color(theme);
        let health_label = self.stats.health_label();
        let health_pct = (self.stats.health * 100.0) as u32;

        div()
            .flex()
            .items_center()
            .gap_2()
            .px_2()
            .py_1()
            .rounded_md()
            .bg(health_color.opacity(0.1))
            // Health icon
            .child(
                div()
                    .text_xs()
                    .child(if self.stats.health > 0.8 {
                        "💚"
                    } else if self.stats.health > 0.5 {
                        "💛"
                    } else {
                        "❤️"
                    })
            )
            // Health bar
            .child(
                div()
                    .w(px(40.0))
                    .h(px(4.0))
                    .rounded_full()
                    .bg(theme.colors.border.opacity(0.3))
                    .child(
                        div()
                            .h_full()
                            .rounded_full()
                            .bg(health_color)
                            .w(pct(health_pct as f32))
                    )
            )
            // Health label
            .child(
                div()
                    .text_xs()
                    .text_color(health_color)
                    .child(health_label)
            )
    }


    pub fn render_onboarding_tip(&self, tip_id: &'static str, icon: &'static str, title: impl Into<SharedString>, description: impl Into<SharedString>, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> Div {
        let title_str: SharedString = title.into();
        let description_str: SharedString = description.into();

        div()
            .flex()
            .items_start()
            .gap_3()
            .p_3()
            .mx_4()
            .my_2()
            .rounded_lg()
            .bg(theme.colors.info.opacity(0.05))
            .border_1()
            .border_color(theme.colors.info.opacity(0.2))
            // Icon
            .child(
                div()
                    .text_lg()
                    .child(icon)
            )
            // Content
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(theme.colors.info)
                                    .child(title_str)
                            )
                            .child(
                                div()
                                    .id(ElementId::Name(format!("dismiss-tip-{}", tip_id).into()))
                                    .w(px(20.0))
                                    .h(px(20.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded_sm()
                                    .cursor_pointer()
                                    .text_xs()
                                    .text_color(theme.colors.text_muted)
                                    .hover(|s| s.bg(theme.colors.surface_hover).text_color(theme.colors.text))
                                    .on_click(cx.listener(move |this, _, _window, cx| {
                                        this.dismiss_tip(tip_id, cx);
                                    }))
                                    .child("×")
                            )
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child(description_str)
                    )
            )
    }


    pub fn render_draft_indicator(&self, theme: &crate::app::theme::Theme) -> Div {
        div()
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py_px()
            .rounded_sm()
            .bg(theme.colors.info.opacity(0.1))
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.info)
                    .child("💾")
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.colors.info)
                    .child("Draft saved")
            )
            .when_some(self.draft_saved_at, |d, saved_at| {
                let elapsed = chrono::Utc::now().signed_duration_since(saved_at);
                let time_str = if elapsed.num_seconds() < 60 {
                    format!("{}s ago", elapsed.num_seconds())
                } else {
                    format!("{}m ago", elapsed.num_minutes())
                };
                d.child(
                    div()
                        .text_xs()
                        .text_color(theme.colors.text_muted)
                        .child(format!("({})", time_str))
                )
            })
    }
}
