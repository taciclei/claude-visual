//! Select component - dropdown selection

use gpui::*;
use gpui::prelude::*;

use super::types::{SelectOption, SelectSize, SelectVariant};

/// Select component - dropdown selection
#[derive(IntoElement)]
pub struct Select {
    id: ElementId,
    options: Vec<SelectOption>,
    selected: Option<SharedString>,
    placeholder: SharedString,
    size: SelectSize,
    variant: SelectVariant,
    disabled: bool,
    searchable: bool,
    clearable: bool,
    open: bool,
    background: Option<gpui::Hsla>,
    border_color: Option<gpui::Hsla>,
    text_color: Option<gpui::Hsla>,
}

impl Select {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            options: Vec::new(),
            selected: None,
            placeholder: "Select...".into(),
            size: SelectSize::default(),
            variant: SelectVariant::default(),
            disabled: false,
            searchable: false,
            clearable: false,
            open: false,
            background: None,
            border_color: None,
            text_color: None,
        }
    }

    pub fn options(mut self, options: Vec<SelectOption>) -> Self {
        self.options = options;
        self
    }

    pub fn selected(mut self, value: impl Into<SharedString>) -> Self {
        self.selected = Some(value.into());
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn size(mut self, size: SelectSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: SelectVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn searchable(mut self, searchable: bool) -> Self {
        self.searchable = searchable;
        self
    }

    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
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
            SelectSize::Small => (28.0, 8.0, 12.0),
            SelectSize::Medium => (36.0, 12.0, 14.0),
            SelectSize::Large => (44.0, 16.0, 16.0),
        }
    }
}

impl RenderOnce for Select {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (height, padding_x, font_size) = self.get_size_styles();
        let bg = self.background.unwrap_or(hsla(0.0, 0.0, 0.15, 1.0));
        let border = self.border_color.unwrap_or(hsla(0.0, 0.0, 0.3, 1.0));
        let text = self.text_color.unwrap_or(hsla(0.0, 0.0, 0.9, 1.0));

        let display_text = self.selected
            .as_ref()
            .and_then(|v| self.options.iter().find(|o| &o.value == v))
            .map(|o| o.label.clone())
            .unwrap_or(self.placeholder.clone());

        let is_placeholder = self.selected.is_none();

        div()
            .id(self.id)
            .h(px(height))
            .px(px(padding_x))
            .flex()
            .items_center()
            .justify_between()
            .gap(px(8.0))
            .rounded(px(6.0))
            .border_1()
            .border_color(border)
            .bg(bg)
            .cursor_pointer()
            .when(self.disabled, |el| el.opacity(0.5).cursor_not_allowed())
            .child(
                div()
                    .text_size(px(font_size))
                    .text_color(if is_placeholder {
                        hsla(0.0, 0.0, 0.5, 1.0)
                    } else {
                        text
                    })
                    .child(display_text)
            )
            .child(
                // Chevron down icon
                div()
                    .text_size(px(10.0))
                    .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                    .child("▼")
            )
    }
}
