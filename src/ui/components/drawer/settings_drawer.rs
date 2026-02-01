//! Settings drawer with sections

use gpui::prelude::*;
use gpui::*;

/// Settings drawer with sections
#[derive(Clone)]
pub struct SettingsDrawer {
    title: String,
    sections: Vec<SettingsSection>,
}

#[derive(Clone)]
pub struct SettingsSection {
    pub title: String,
    pub items: Vec<SettingsItem>,
}

#[derive(Clone)]
pub struct SettingsItem {
    pub label: String,
    pub description: Option<String>,
    pub item_type: SettingsItemType,
}

#[derive(Clone)]
pub enum SettingsItemType {
    Toggle(bool),
    Value(String),
    Link,
}

impl SettingsDrawer {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            sections: Vec::new(),
        }
    }

    pub fn section(mut self, title: impl Into<String>, items: Vec<SettingsItem>) -> Self {
        self.sections.push(SettingsSection {
            title: title.into(),
            items,
        });
        self
    }
}

impl RenderOnce for SettingsDrawer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let surface = hsla(0.0, 0.0, 0.12, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let text_muted = hsla(0.0, 0.0, 0.5, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);

        div()
            .w(px(360.0))
            .h_full()
            .bg(surface)
            .border_l_1()
            .border_color(border)
            .flex()
            .flex_col()
            // Header
            .child(
                div()
                    .w_full()
                    .px_4()
                    .py_4()
                    .border_b_1()
                    .border_color(border)
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(text)
                            .child(self.title),
                    )
                    .child(
                        div()
                            .size(px(28.0))
                            .rounded(px(4.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(text_muted)
                            .cursor_pointer()
                            .hover(|s| s.bg(hsla(0.0, 0.0, 0.18, 1.0)))
                            .child("×"),
                    ),
            )
            // Sections
            .child(
                div()
                    .flex_1()
                    .id("scroll-settings-drawer")
                    .overflow_y_scroll()
                    .children(self.sections.into_iter().map(|section| {
                        div()
                            .w_full()
                            .py_4()
                            .border_b_1()
                            .border_color(border)
                            // Section title
                            .child(
                                div()
                                    .px_4()
                                    .pb_2()
                                    .text_xs()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(text_muted)
                                    .child(section.title.to_uppercase()),
                            )
                            // Items
                            .children(section.items.into_iter().map(|item| {
                                div()
                                    .w_full()
                                    .px_4()
                                    .py_3()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .cursor_pointer()
                                    .hover(|s| s.bg(hsla(0.0, 0.0, 0.15, 1.0)))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_1()
                                            .child(
                                                div().text_sm().text_color(text).child(item.label),
                                            )
                                            .when_some(item.description, |d, desc| {
                                                d.child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(text_muted)
                                                        .child(desc),
                                                )
                                            }),
                                    )
                                    .child(match item.item_type {
                                        SettingsItemType::Toggle(on) => {
                                            // Simple toggle indicator
                                            div()
                                                .w(px(40.0))
                                                .h(px(22.0))
                                                .rounded(px(11.0))
                                                .bg(if on {
                                                    accent
                                                } else {
                                                    hsla(0.0, 0.0, 0.3, 1.0)
                                                })
                                                .flex()
                                                .items_center()
                                                .child(
                                                    div()
                                                        .size(px(18.0))
                                                        .rounded_full()
                                                        .bg(gpui::white())
                                                        .ml(if on { px(20.0) } else { px(2.0) }),
                                                )
                                                .into_any_element()
                                        }
                                        SettingsItemType::Value(val) => div()
                                            .text_sm()
                                            .text_color(text_muted)
                                            .child(val)
                                            .into_any_element(),
                                        SettingsItemType::Link => div()
                                            .text_color(text_muted)
                                            .child("→")
                                            .into_any_element(),
                                    })
                            }))
                    })),
            )
    }
}
