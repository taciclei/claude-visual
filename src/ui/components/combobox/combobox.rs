//! Combobox component - searchable dropdown

use super::types::*;
use gpui::prelude::*;
use gpui::*;

/// Combobox component - searchable dropdown
#[derive(IntoElement)]
pub struct Combobox {
    id: ElementId,
    items: Vec<ComboboxItem>,
    selected: Option<SharedString>,
    query: SharedString,
    placeholder: SharedString,
    pub(crate) size: ComboboxSize,
    pub(crate) mode: ComboboxMode,
    disabled: bool,
    loading: bool,
    open: bool,
    no_results_text: SharedString,
    background: Option<gpui::Hsla>,
    border_color: Option<gpui::Hsla>,
    text_color: Option<gpui::Hsla>,
}

impl Combobox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            selected: None,
            query: "".into(),
            placeholder: "Search...".into(),
            size: ComboboxSize::default(),
            mode: ComboboxMode::default(),
            disabled: false,
            loading: false,
            open: false,
            no_results_text: "No results found".into(),
            background: None,
            border_color: None,
            text_color: None,
        }
    }

    pub fn items(mut self, items: Vec<ComboboxItem>) -> Self {
        self.items = items;
        self
    }

    pub fn selected(mut self, value: impl Into<SharedString>) -> Self {
        self.selected = Some(value.into());
        self
    }

    pub fn query(mut self, query: impl Into<SharedString>) -> Self {
        self.query = query.into();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn size(mut self, size: ComboboxSize) -> Self {
        self.size = size;
        self
    }

    pub fn mode(mut self, mode: ComboboxMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn no_results_text(mut self, text: impl Into<SharedString>) -> Self {
        self.no_results_text = text.into();
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = Some(color);
        self
    }

    pub fn border_color(mut self, color: gpui::Hsla) -> Self {
        self.border_color = Some(color);
        self
    }

    pub fn text_color(mut self, color: gpui::Hsla) -> Self {
        self.text_color = Some(color);
        self
    }

    fn get_size_styles(&self) -> (f32, f32, f32) {
        match self.size {
            ComboboxSize::Small => (28.0, 8.0, 12.0),
            ComboboxSize::Medium => (36.0, 12.0, 14.0),
            ComboboxSize::Large => (44.0, 16.0, 16.0),
        }
    }
}

impl RenderOnce for Combobox {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (height, padding_x, font_size) = self.get_size_styles();
        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.15, 1.0));
        let border = self.border_color.unwrap_or(hsla(0.0, 0.0, 0.3, 1.0));
        let text = self.text_color.unwrap_or(hsla(0.0, 0.0, 0.9, 1.0));

        let display_value = if !self.query.is_empty() {
            self.query.clone()
        } else if let Some(ref selected) = self.selected {
            self.items
                .iter()
                .find(|i| &i.value == selected)
                .map(|i| i.label.clone())
                .unwrap_or(self.placeholder.clone())
        } else {
            self.placeholder.clone()
        };

        let is_placeholder = self.query.is_empty() && self.selected.is_none();

        div().id(self.id).flex().flex_col().w_full().child(
            // Input container
            div()
                .h(px(height))
                .px(px(padding_x))
                .flex()
                .items_center()
                .gap(px(8.0))
                .rounded(px(6.0))
                .border_1()
                .border_color(border)
                .bg(bg)
                .when(self.open, |el| el.rounded_b_none())
                .when(self.disabled, |el| el.opacity(0.5).cursor_not_allowed())
                .child(
                    // Search icon
                    div()
                        .text_size(px(14.0))
                        .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                        .child("🔍"),
                )
                .child(
                    // Input text
                    div()
                        .flex_1()
                        .text_size(px(font_size))
                        .text_color(if is_placeholder {
                            hsla(0.0, 0.0, 0.5, 1.0)
                        } else {
                            text
                        })
                        .child(display_value),
                )
                .when(self.loading, |el| {
                    el.child(
                        div()
                            .text_size(px(12.0))
                            .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                            .child("⏳"),
                    )
                })
                .when(self.selected.is_some() && !self.loading, |el| {
                    el.child(
                        div()
                            .text_size(px(12.0))
                            .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                            .cursor_pointer()
                            .child("✕"),
                    )
                }),
        )
    }
}
