//! Load more trigger component

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Load more trigger component
#[derive(IntoElement)]
pub struct LoadMoreTrigger {
    id: ElementId,
    pub(crate) loading: bool,
    pub(crate) has_more: bool,
    button_text: SharedString,
    loading_text: SharedString,
    pub(crate) variant: LoadMoreVariant,
    background: gpui::Hsla,
}

impl LoadMoreTrigger {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            loading: false,
            has_more: true,
            button_text: "Load more".into(),
            loading_text: "Loading...".into(),
            variant: LoadMoreVariant::default(),
            background: rgba(0x00000000).into(),
        }
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn has_more(mut self, has_more: bool) -> Self {
        self.has_more = has_more;
        self
    }

    pub fn button_text(mut self, text: impl Into<SharedString>) -> Self {
        self.button_text = text.into();
        self
    }

    pub fn loading_text(mut self, text: impl Into<SharedString>) -> Self {
        self.loading_text = text.into();
        self
    }

    pub fn variant(mut self, variant: LoadMoreVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = color;
        self
    }
}

impl RenderOnce for LoadMoreTrigger {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        if !self.has_more {
            return div().id(self.id);
        }

        let content = if self.loading {
            self.loading_text.clone()
        } else {
            self.button_text.clone()
        };

        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .py_4()
            .bg(self.background)
            .child(match self.variant {
                LoadMoreVariant::Button => div()
                    .px_4()
                    .py_2()
                    .rounded_md()
                    .bg(rgb(0x3b82f6))
                    .text_sm()
                    .text_color(rgb(0xffffff))
                    .cursor_pointer()
                    .when(self.loading, |d| d.opacity(0.7).cursor_default())
                    .child(content),
                LoadMoreVariant::Link => div()
                    .text_sm()
                    .text_color(rgb(0x3b82f6))
                    .cursor_pointer()
                    .when(self.loading, |d| d.opacity(0.7))
                    .child(content),
                LoadMoreVariant::Auto => div()
                    .h_1()
                    .w_full()
                    // Invisible trigger for intersection observer
                    .when(self.loading, |d| {
                        d.child(
                            div()
                                .size_4()
                                .rounded_full()
                                .border_2()
                                .border_color(rgba(0x3333331a))
                                .border_color(rgb(0x3b82f6)),
                        )
                    }),
            })
    }
}
