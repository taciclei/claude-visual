//! Infinite scroll container with load-more detection

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Infinite scroll container with load-more detection
#[derive(IntoElement)]
pub struct InfiniteScroll {
    id: ElementId,
    pub(crate) direction: ScrollDirection,
    state: LoadingState,
    pub(crate) threshold: f32,
    pub(crate) items_per_page: usize,
    total_items: Option<usize>,
    current_count: usize,
    loading_text: SharedString,
    end_text: SharedString,
    error_text: SharedString,
    show_loader: bool,
    background: gpui::Hsla,
}

impl InfiniteScroll {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            direction: ScrollDirection::default(),
            state: LoadingState::default(),
            threshold: 200.0,
            items_per_page: 20,
            total_items: None,
            current_count: 0,
            loading_text: "Loading more...".into(),
            end_text: "No more items".into(),
            error_text: "Failed to load".into(),
            show_loader: true,
            background: rgba(0x00000000).into(),
        }
    }

    pub fn direction(mut self, direction: ScrollDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn state(mut self, state: LoadingState) -> Self {
        self.state = state;
        self
    }

    pub fn threshold(mut self, threshold: f32) -> Self {
        self.threshold = threshold;
        self
    }

    pub fn items_per_page(mut self, count: usize) -> Self {
        self.items_per_page = count;
        self
    }

    pub fn total_items(mut self, total: usize) -> Self {
        self.total_items = Some(total);
        self
    }

    pub fn current_count(mut self, count: usize) -> Self {
        self.current_count = count;
        self
    }

    pub fn loading_text(mut self, text: impl Into<SharedString>) -> Self {
        self.loading_text = text.into();
        self
    }

    pub fn end_text(mut self, text: impl Into<SharedString>) -> Self {
        self.end_text = text.into();
        self
    }

    pub fn error_text(mut self, text: impl Into<SharedString>) -> Self {
        self.error_text = text.into();
        self
    }

    pub fn show_loader(mut self, show: bool) -> Self {
        self.show_loader = show;
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = color;
        self
    }

    fn render_loader(&self) -> Div {
        let text_color: Hsla = rgba(0x888888ff).into();

        match self.state {
            LoadingState::Idle => div(),
            LoadingState::Loading | LoadingState::LoadingMore => div()
                .flex()
                .items_center()
                .justify_center()
                .py_4()
                .gap_2()
                .child(
                    // Spinner
                    div()
                        .size_4()
                        .rounded_full()
                        .border_2()
                        .border_color(rgba(0x3333331a))
                        .border_color(rgb(0x3b82f6)),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(text_color)
                        .child(self.loading_text.clone()),
                ),
            LoadingState::Error => div()
                .flex()
                .items_center()
                .justify_center()
                .py_4()
                .gap_2()
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(0xef4444))
                        .child(self.error_text.clone()),
                )
                .child(
                    div()
                        .px_3()
                        .py_1()
                        .rounded_md()
                        .bg(rgb(0x3b82f6))
                        .text_sm()
                        .text_color(rgb(0xffffff))
                        .cursor_pointer()
                        .child("Retry"),
                ),
            LoadingState::EndReached => div()
                .flex()
                .items_center()
                .justify_center()
                .py_4()
                .child(
                    div()
                        .text_sm()
                        .text_color(text_color)
                        .child(self.end_text.clone()),
                ),
        }
    }
}

impl RenderOnce for InfiniteScroll {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let has_more = self
            .total_items
            .map(|total| self.current_count < total)
            .unwrap_or(true);
        let id = self.id.clone();

        div()
            .id(id)
            .flex()
            .flex_col()
            .size_full()
            .overflow_y_scroll()
            .bg(self.background)
            .when(self.show_loader && (self.state != LoadingState::Idle || !has_more), |d| {
                d.child(self.render_loader())
            })
    }
}
