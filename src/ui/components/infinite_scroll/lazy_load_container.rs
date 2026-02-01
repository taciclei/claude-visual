//! Lazy load container that loads content when visible

use gpui::prelude::*;
use gpui::*;

/// Lazy load container that loads content when visible
#[derive(IntoElement)]
pub struct LazyLoadContainer {
    id: ElementId,
    pub(crate) loaded: bool,
    loading: bool,
    pub(crate) placeholder_height: f32,
    fade_in: bool,
    background: gpui::Hsla,
}

impl LazyLoadContainer {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            loaded: false,
            loading: false,
            placeholder_height: 200.0,
            fade_in: true,
            background: rgba(0x00000000).into(),
        }
    }

    pub fn loaded(mut self, loaded: bool) -> Self {
        self.loaded = loaded;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn placeholder_height(mut self, height: f32) -> Self {
        self.placeholder_height = height;
        self
    }

    pub fn fade_in(mut self, fade: bool) -> Self {
        self.fade_in = fade;
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = color;
        self
    }
}

impl RenderOnce for LazyLoadContainer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .w_full()
            .min_h(px(self.placeholder_height))
            .bg(self.background)
            .when(!self.loaded && !self.loading, |d| {
                // Placeholder skeleton
                d.child(
                    div()
                        .size_full()
                        .min_h(px(self.placeholder_height))
                        .bg(rgba(0x8888881a))
                        .rounded_md(),
                )
            })
            .when(self.loading, |d| {
                // Loading state
                d.flex().items_center().justify_center().child(
                    div()
                        .size_6()
                        .rounded_full()
                        .border_2()
                        .border_color(rgba(0x3333331a))
                        .border_color(rgb(0x3b82f6)),
                )
            })
    }
}
