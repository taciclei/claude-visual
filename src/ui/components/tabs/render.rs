//! Render implementation for Tabs component

use gpui::prelude::*;
use gpui::*;

use super::tabs_component::Tabs;
use super::types::*;

impl Render for Tabs {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let height = self.size.height();
        let font_size = self.size.font_size();
        let padding_x = self.size.padding_x();

        let is_underline = matches!(self.style, TabsStyle::Underline);
        let is_pill = matches!(self.style, TabsStyle::Pill);
        let is_boxed = matches!(self.style, TabsStyle::Boxed);

        div()
            .id("tabs")
            .w_full()
            .flex()
            .flex_col()
            // Tab list
            .child(
                div()
                    .flex()
                    .items_center()
                    .when(self.full_width, |d| d.w_full())
                    .when(is_underline, |d| {
                        d.border_b_1().border_color(theme.colors.border)
                    })
                    .when(is_pill, |d| d.p_1().rounded_lg().bg(theme.colors.surface))
                    .when(is_boxed, |d| {
                        d.border_1()
                            .border_color(theme.colors.border)
                            .rounded_lg()
                            .overflow_hidden()
                    })
                    // Tab items
                    .children(self.tabs.iter().enumerate().map(|(i, tab)| {
                        let is_active = self.active.as_ref() == Some(&tab.id);
                        let tab_id = tab.id.clone();
                        let is_closable = tab.closable || self.closable;

                        div()
                            .id(SharedString::from(format!("tab-{}", i)))
                            .h(px(height))
                            .px(px(padding_x))
                            .when(self.full_width, |d| d.flex_1())
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap_2()
                            // Style-specific appearances
                            .when(is_underline, |d| {
                                d.when(is_active, |d| {
                                    d.text_color(theme.colors.accent)
                                        .border_b_2()
                                        .border_color(theme.colors.accent)
                                        .mb(px(-1.0)) // Overlap with container border
                                })
                                .when(!is_active, |d| {
                                    d.text_color(theme.colors.text_muted)
                                        .border_b_2()
                                        .border_color(gpui::transparent_black())
                                })
                            })
                            .when(is_pill, |d| {
                                d.rounded(px(6.0))
                                    .when(is_active, |d| {
                                        d.bg(theme.colors.background)
                                            .text_color(theme.colors.text)
                                            .shadow_sm()
                                    })
                                    .when(!is_active, |d| d.text_color(theme.colors.text_muted))
                            })
                            .when(is_boxed, |d| {
                                d.when(is_active, |d| {
                                    d.bg(theme.colors.accent.opacity(0.15))
                                        .text_color(theme.colors.accent)
                                })
                                .when(!is_active, |d| {
                                    d.bg(theme.colors.surface)
                                        .text_color(theme.colors.text_muted)
                                })
                                .when(i > 0, |d| d.border_l_1().border_color(theme.colors.border))
                            })
                            .when(matches!(self.style, TabsStyle::Minimal), |d| {
                                d.when(is_active, |d| {
                                    d.text_color(theme.colors.text)
                                        .font_weight(FontWeight::MEDIUM)
                                })
                                .when(!is_active, |d| d.text_color(theme.colors.text_muted))
                            })
                            // Interactivity
                            .when(!tab.disabled, |d| {
                                d.cursor_pointer()
                                    .when(!is_active && !is_pill && !is_boxed, |d| {
                                        d.hover(|s| s.text_color(theme.colors.text))
                                    })
                                    .when(!is_active && is_pill, |d| {
                                        d.hover(|s| s.bg(theme.colors.surface_hover))
                                    })
                                    .when(!is_active && is_boxed, |d| {
                                        d.hover(|s| s.bg(theme.colors.surface_hover))
                                    })
                            })
                            .when(tab.disabled, |d| d.opacity(0.5))
                            .when(!tab.disabled, |d| {
                                d.on_click(cx.listener(move |this, _, _window, cx| {
                                    this.select(tab_id.clone(), cx);
                                }))
                            })
                            // Icon
                            .when_some(tab.icon.clone(), |d, icon| {
                                d.child(div().text_size(px(font_size)).child(icon))
                            })
                            // Label
                            .child(div().text_size(px(font_size)).child(tab.label.clone()))
                            // Badge
                            .when_some(tab.badge, |d, count| {
                                d.child(
                                    div()
                                        .h(px(16.0))
                                        .min_w(px(16.0))
                                        .px_1()
                                        .rounded_full()
                                        .bg(theme.colors.error)
                                        .text_color(gpui::white())
                                        .text_size(px(10.0))
                                        .font_weight(FontWeight::MEDIUM)
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(if count > 99 {
                                            "99+".to_string()
                                        } else {
                                            count.to_string()
                                        }),
                                )
                            })
                            // Close button
                            .when(is_closable && !tab.disabled, |d| {
                                let close_tab_id = tab.id.clone();
                                d.child(
                                    div()
                                        .id(SharedString::from(format!("tab-close-{}", i)))
                                        .ml_1()
                                        .size(px(16.0))
                                        .rounded(px(4.0))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_size(px(10.0))
                                        .hover(|s| s.bg(theme.colors.surface_hover))
                                        .on_click(cx.listener(move |_this, _, _window, cx| {
                                            cx.emit(TabsEvent::CloseRequested(
                                                close_tab_id.clone(),
                                            ));
                                        }))
                                        .child("×"),
                                )
                            })
                    })),
            )
    }
}
