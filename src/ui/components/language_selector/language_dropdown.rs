//! Language dropdown menu component

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Language dropdown menu
#[derive(IntoElement)]
pub struct LanguageDropdown {
    id: ElementId,
    languages: Vec<Language>,
    selected_code: Option<SharedString>,
    show_native_name: bool,
    show_code: bool,
    max_height: f32,
}

impl LanguageDropdown {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            languages: vec![
                Language::english(),
                Language::french(),
                Language::spanish(),
                Language::german(),
                Language::portuguese(),
                Language::chinese(),
                Language::japanese(),
            ],
            selected_code: None,
            show_native_name: true,
            show_code: true,
            max_height: 300.0,
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

    pub fn show_native_name(mut self, show: bool) -> Self {
        self.show_native_name = show;
        self
    }

    pub fn show_code(mut self, show: bool) -> Self {
        self.show_code = show;
        self
    }

    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = height;
        self
    }
}

impl RenderOnce for LanguageDropdown {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .flex_col()
            .w(px(240.0))
            .max_h(px(self.max_height))
            .id("scroll-language-dropdown")
            .overflow_y_scroll()
            .bg(hsla(0.0, 0.0, 0.12, 1.0))
            .border_1()
            .border_color(hsla(0.0, 0.0, 0.2, 1.0))
            .rounded(px(8.0))
            .shadow_lg()
            .children(self.languages.iter().map(|lang| {
                let is_selected = self.selected_code.as_ref() == Some(&lang.code);

                div()
                    .flex()
                    .items_center()
                    .gap(px(10.0))
                    .px(px(12.0))
                    .py(px(10.0))
                    .bg(if is_selected {
                        hsla(0.6, 0.5, 0.4, 0.2)
                    } else {
                        hsla(0.0, 0.0, 0.0, 0.0)
                    })
                    .cursor_pointer()
                    // Flag
                    .when_some(lang.flag.clone(), |el, flag| {
                        el.child(div().text_size(px(20.0)).child(flag))
                    })
                    // Names
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(hsla(0.0, 0.0, 0.9, 1.0))
                                    .child(lang.name.clone()),
                            )
                            .when(
                                self.show_native_name && lang.name != lang.native_name,
                                |el| {
                                    el.child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(hsla(0.0, 0.0, 0.6, 1.0))
                                            .child(lang.native_name.clone()),
                                    )
                                },
                            ),
                    )
                    // Code
                    .when(self.show_code, |el| {
                        el.child(
                            div()
                                .text_size(px(11.0))
                                .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                                .child(lang.code.as_ref().to_uppercase()),
                        )
                    })
                    // Selected indicator
                    .when(is_selected, |el| {
                        el.child(
                            div()
                                .text_size(px(14.0))
                                .text_color(hsla(0.6, 0.7, 0.5, 1.0))
                                .child("✓"),
                        )
                    })
            }))
    }
}
