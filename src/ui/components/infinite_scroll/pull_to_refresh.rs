//! Pull to refresh component

use gpui::*;
use gpui::prelude::*;
use super::types::*;

/// Pull to refresh component
#[derive(IntoElement)]
pub struct PullToRefresh {
    id: ElementId,
    pub(crate) state: PullState,
    pub(crate) pull_distance: f32,
    threshold: f32,
    max_pull: f32,
    pull_text: SharedString,
    release_text: SharedString,
    refreshing_text: SharedString,
    indicator_color: gpui::Hsla,
    background: gpui::Hsla,
}

impl PullToRefresh {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            state: PullState::default(),
            pull_distance: 0.0,
            threshold: 80.0,
            max_pull: 120.0,
            pull_text: "Pull to refresh".into(),
            release_text: "Release to refresh".into(),
            refreshing_text: "Refreshing...".into(),
            indicator_color: rgb(0x3b82f6).into(),
            background: rgba(0x00000000).into(),
        }
    }

    pub fn state(mut self, state: PullState) -> Self {
        self.state = state;
        self
    }

    pub fn pull_distance(mut self, distance: f32) -> Self {
        self.pull_distance = distance;
        self
    }

    pub fn threshold(mut self, threshold: f32) -> Self {
        self.threshold = threshold;
        self
    }

    pub fn max_pull(mut self, max: f32) -> Self {
        self.max_pull = max;
        self
    }

    pub fn pull_text(mut self, text: impl Into<SharedString>) -> Self {
        self.pull_text = text.into();
        self
    }

    pub fn release_text(mut self, text: impl Into<SharedString>) -> Self {
        self.release_text = text.into();
        self
    }

    pub fn refreshing_text(mut self, text: impl Into<SharedString>) -> Self {
        self.refreshing_text = text.into();
        self
    }

    pub fn indicator_color(mut self, color: gpui::Hsla) -> Self {
        self.indicator_color = color;
        self
    }

    pub fn background(mut self, color: gpui::Hsla) -> Self {
        self.background = color;
        self
    }
}

impl RenderOnce for PullToRefresh {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text_color: Hsla = rgba(0x888888ff).into();
        let visible_height = self.pull_distance.min(self.max_pull);
        let progress = (self.pull_distance / self.threshold).min(1.0);

        let status_text = match self.state {
            PullState::Idle => "",
            PullState::Pulling => &self.pull_text,
            PullState::ReadyToRefresh => &self.release_text,
            PullState::Refreshing => &self.refreshing_text,
        };

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .size_full()
            .bg(self.background)
            .child(
                // Pull indicator
                div()
                    .w_full()
                    .h(px(visible_height))
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_end()
                    .pb_2()
                    .when(self.state != PullState::Idle, |d| {
                        d.child(
                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .gap_2()
                                .child(
                                    // Progress indicator
                                    div()
                                        .size_6()
                                        .rounded_full()
                                        .border_2()
                                        .border_color(rgba(0x3333331a))
                                        .when(self.state == PullState::Refreshing, |d| {
                                            d.border_color(self.indicator_color)
                                        })
                                        .when(self.state != PullState::Refreshing, |d| {
                                            // Show progress arc
                                            d.bg(self.indicator_color.opacity(progress))
                                        }),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(text_color)
                                        .child(status_text.to_string()),
                                ),
                        )
                    }),
            )
    }
}
