//! Form component - container for form fields

use super::types::{FormLayout, FormSize};
use gpui::prelude::*;
use gpui::*;

#[derive(IntoElement)]
pub struct Form {
    id: ElementId,
    children: Vec<gpui::AnyElement>,
    layout: FormLayout,
    size: FormSize,
    disabled: bool,
    loading: bool,
    spacing: f32,
}

impl Form {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
            layout: FormLayout::default(),
            size: FormSize::default(),
            disabled: false,
            loading: false,
            spacing: 16.0,
        }
    }

    pub fn children(mut self, children: Vec<impl IntoElement>) -> Self {
        self.children = children.into_iter().map(|c| c.into_any_element()).collect();
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn layout(mut self, layout: FormLayout) -> Self {
        self.layout = layout;
        self
    }

    pub fn size(mut self, size: FormSize) -> Self {
        self.size = size;
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

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl RenderOnce for Form {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut form = div().id(self.id);

        form = match self.layout {
            FormLayout::Vertical => form.flex().flex_col().gap(px(self.spacing)),
            FormLayout::Horizontal => form.flex().flex_row().flex_wrap().gap(px(self.spacing)),
            FormLayout::Inline => form.flex().flex_row().items_end().gap(px(12.0)),
        };

        if self.disabled || self.loading {
            form = form.opacity(0.6);
        }

        form.children(self.children)
    }
}
