//! Settings sidebar with tabs

use gpui::*;
use gpui::prelude::*;
use crate::app::theme::Theme;
use super::super::core::SettingsModal;
use super::super::types::SettingsTab;

impl SettingsModal {
    pub(super) fn render_sidebar(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let accent = theme.colors.accent;
        let surface_hover = theme.colors.surface_hover;
        let text_color = theme.colors.text;
        let text_muted = theme.colors.text_muted;

        div()
            .w(px(180.0))
            .h_full()
            .border_r_1()
            .border_color(theme.colors.border)
            .p_2()
            .flex()
            .flex_col()
            .gap_1()
            .children(SettingsTab::all().iter().map(|&tab| {
                let is_active = self.active_tab == tab;
                let on_click = cx.listener(move |this, _, _window, cx| {
                    this.switch_tab(tab, cx);
                });

                div()
                    .id(SharedString::from(format!("settings-tab-{}", tab.label())))
                    .w_full()
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .cursor_pointer()
                    .text_sm()
                    .when(is_active, |d| {
                        d.bg(accent.opacity(0.15))
                            .text_color(accent)
                            .font_weight(FontWeight::MEDIUM)
                    })
                    .when(!is_active, |d| {
                        d.text_color(text_muted)
                            .hover(move |s| {
                                s.bg(surface_hover)
                                    .text_color(text_color)
                            })
                    })
                    .on_click(on_click)
                    .child(tab.label())
            }))
    }
}
