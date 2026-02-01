//! Language selector component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Language selector component
#[derive(IntoElement)]
pub struct LanguageSelector {
    id: ElementId,
    languages: Vec<Language>,
    selected_code: Option<SharedString>,
    variant: LanguageSelectorVariant,
    size: LanguageSelectorSize,
    show_native_name: bool,
    show_flag: bool,
}

impl LanguageSelector {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            languages: vec![
                Language::english(),
                Language::french(),
                Language::spanish(),
                Language::german(),
            ],
            selected_code: Some("en".into()),
            variant: LanguageSelectorVariant::default(),
            size: LanguageSelectorSize::default(),
            show_native_name: false,
            show_flag: true,
        }
    }

    pub fn languages(mut self, languages: Vec<Language>) -> Self {
        self.languages = languages;
        self
    }

    pub fn selected(mut self, code: impl Into<SharedString>) -> Self {
        self.selected_code = Some(code.into());
        self
    }

    pub fn variant(mut self, variant: LanguageSelectorVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: LanguageSelectorSize) -> Self {
        self.size = size;
        self
    }

    pub fn show_native_name(mut self, show: bool) -> Self {
        self.show_native_name = show;
        self
    }

    pub fn show_flag(mut self, show: bool) -> Self {
        self.show_flag = show;
        self
    }

    fn get_selected(&self) -> Option<&Language> {
        self.selected_code
            .as_ref()
            .and_then(|code| self.languages.iter().find(|l| &l.code == code))
    }
}

impl RenderOnce for LanguageSelector {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let height = self.size.height();
        let font_size = self.size.font_size();
        let id = self.id.clone();
        let selected = self.get_selected();

        match self.variant {
            LanguageSelectorVariant::Dropdown => div()
                .id(id)
                .flex()
                .items_center()
                .justify_between()
                .h(px(height))
                .px(px(12.0))
                .bg(hsla(0.0, 0.0, 0.12, 1.0))
                .border_1()
                .border_color(hsla(0.0, 0.0, 0.2, 1.0))
                .rounded(px(8.0))
                .cursor_pointer()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .when(self.show_flag, |el| {
                            if let Some(lang) = &selected {
                                if let Some(flag) = &lang.flag {
                                    return el.child(
                                        div().text_size(px(font_size + 4.0)).child(flag.clone()),
                                    );
                                }
                            }
                            el
                        })
                        .when_some(selected.clone(), |el, lang| {
                            el.child(
                                div()
                                    .text_size(px(font_size))
                                    .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                                    .child(if self.show_native_name {
                                        lang.native_name.clone()
                                    } else {
                                        lang.name.clone()
                                    }),
                            )
                        }),
                )
                .child(
                    div()
                        .text_size(px(10.0))
                        .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                        .child("▼"),
                ),
            LanguageSelectorVariant::Button => div()
                .id(id.clone())
                .flex()
                .items_center()
                .gap(px(6.0))
                .h(px(height))
                .px(px(12.0))
                .bg(hsla(0.0, 0.0, 0.15, 1.0))
                .border_1()
                .border_color(hsla(0.0, 0.0, 0.25, 1.0))
                .rounded(px(8.0))
                .cursor_pointer()
                .when(self.show_flag, |el| {
                    if let Some(lang) = &selected {
                        if let Some(flag) = &lang.flag {
                            return el
                                .child(div().text_size(px(font_size + 2.0)).child(flag.clone()));
                        }
                    }
                    el
                })
                .child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(gpui::FontWeight::MEDIUM)
                        .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                        .child(
                            selected
                                .map(|l| l.code.as_ref())
                                .unwrap_or("EN")
                                .to_uppercase(),
                        ),
                ),
            LanguageSelectorVariant::Minimal => div()
                .id(id)
                .flex()
                .items_center()
                .gap(px(4.0))
                .cursor_pointer()
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(hsla(0.0, 0.0, 0.7, 1.0))
                        .child("🌐"),
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(hsla(0.0, 0.0, 0.7, 1.0))
                        .child(
                            selected
                                .map(|l| l.code.as_ref())
                                .unwrap_or("en")
                                .to_uppercase(),
                        ),
                ),
            LanguageSelectorVariant::Flags => {
                div()
                    .id(self.id)
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .children(self.languages.iter().map(|lang| {
                        let is_selected = self.selected_code.as_ref() == Some(&lang.code);
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(32.0))
                            .h(px(32.0))
                            .rounded(px(4.0))
                            .bg(if is_selected {
                                hsla(0.6, 0.5, 0.4, 0.2)
                            } else {
                                hsla(0.0, 0.0, 0.0, 0.0)
                            })
                            .border_1()
                            .border_color(if is_selected {
                                hsla(0.6, 0.7, 0.5, 1.0)
                            } else {
                                hsla(0.0, 0.0, 0.0, 0.0)
                            })
                            .cursor_pointer()
                            .child(div().text_size(px(18.0)).child(
                                lang.flag.clone().unwrap_or_else(|| {
                                    lang.code.chars().take(2).collect::<String>().into()
                                }),
                            ))
                    }))
            }
        }
    }
}
