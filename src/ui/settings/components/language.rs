use super::super::SettingsModal;
use crate::app::settings::LanguageSetting;
use crate::i18n::{i18n, Locale};
use gpui::prelude::*;
use gpui::*;

impl SettingsModal {
    /// Render the language selector
    pub(crate) fn render_language_selector(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let current_setting = &self.pending.language;
        let is_auto = current_setting.is_auto();

        div()
            .flex()
            .flex_col()
            .gap_3()
            // Auto-detect toggle
            .child(
                div()
                    .id("lang-auto")
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .cursor_pointer()
                    .border_2()
                    .when(is_auto, |d| {
                        d.border_color(theme.colors.accent)
                            .bg(theme.colors.accent.opacity(0.1))
                    })
                    .when(!is_auto, |d| {
                        d.border_color(theme.colors.border)
                            .hover(|s| s.bg(theme.colors.surface_hover))
                    })
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.pending.language = LanguageSetting::Auto;
                        this.mark_changed(cx);
                    }))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(if is_auto {
                                        FontWeight::SEMIBOLD
                                    } else {
                                        FontWeight::NORMAL
                                    })
                                    .child("Auto-detect"),
                            )
                            .child(div().text_xs().text_color(theme.colors.text_muted).child(
                                format!(
                                    "Currently: {}",
                                    crate::i18n::detect_system_locale().native_name()
                                ),
                            )),
                    )
                    .when(is_auto, |d| {
                        d.child(div().text_sm().text_color(theme.colors.accent).child("✓"))
                    }),
            )
            // Available languages
            .child(
                div().flex().flex_col().gap_1().children(
                    Locale::all()
                        .iter()
                        .filter(|l| i18n().has_locale(**l))
                        .map(|&locale| {
                            let is_selected = match current_setting {
                                LanguageSetting::Auto => false,
                                LanguageSetting::Specific(tag) => {
                                    Locale::from_tag(tag) == Some(locale)
                                }
                            };
                            let tag = locale.language_tag().to_string();

                            div()
                                .id(SharedString::from(format!(
                                    "lang-{}",
                                    locale.language_tag()
                                )))
                                .flex()
                                .items_center()
                                .justify_between()
                                .px_3()
                                .py_2()
                                .rounded_md()
                                .cursor_pointer()
                                .border_2()
                                .when(is_selected, |d| {
                                    d.border_color(theme.colors.accent)
                                        .bg(theme.colors.accent.opacity(0.1))
                                })
                                .when(!is_selected, |d| {
                                    d.border_color(theme.colors.border)
                                        .hover(|s| s.bg(theme.colors.surface_hover))
                                })
                                .on_click(cx.listener(move |this, _, _window, cx| {
                                    this.pending.language = LanguageSetting::Specific(tag.clone());
                                    this.mark_changed(cx);
                                }))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(div().text_base().child(locale.flag_emoji()))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .font_weight(if is_selected {
                                                            FontWeight::SEMIBOLD
                                                        } else {
                                                            FontWeight::NORMAL
                                                        })
                                                        .child(locale.native_name()),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(theme.colors.text_muted)
                                                        .child(locale.english_name()),
                                                ),
                                        ),
                                )
                                .when(is_selected, |d| {
                                    d.child(
                                        div().text_sm().text_color(theme.colors.accent).child("✓"),
                                    )
                                })
                        }),
                ),
            )
    }
}
