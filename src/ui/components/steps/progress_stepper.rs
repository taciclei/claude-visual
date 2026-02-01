use gpui::*;
use gpui::prelude::*;
use crate::ui::pct;

/// Progress stepper for forms
#[derive(IntoElement)]
pub struct ProgressStepper {
    id: ElementId,
    total_steps: usize,
    current_step: usize,
    show_labels: bool,
    labels: Vec<SharedString>,
    completed_color: gpui::Hsla,
    current_color: gpui::Hsla,
    pending_color: gpui::Hsla,
}

impl ProgressStepper {
    pub fn new(id: impl Into<ElementId>, total: usize) -> Self {
        Self {
            id: id.into(),
            total_steps: total,
            current_step: 0,
            show_labels: false,
            labels: Vec::new(),
            completed_color: rgb(0x22c55e).into(),
            current_color: rgb(0x3b82f6).into(),
            pending_color: rgba(0x8888884d).into(),
        }
    }

    pub fn current_step(mut self, step: usize) -> Self {
        self.current_step = step;
        self
    }

    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    pub fn labels(mut self, labels: Vec<impl Into<SharedString>>) -> Self {
        self.labels = labels.into_iter().map(|l| l.into()).collect();
        self.show_labels = true;
        self
    }

    pub fn completed_color(mut self, color: gpui::Hsla) -> Self {
        self.completed_color = color;
        self
    }

    pub fn current_color(mut self, color: gpui::Hsla) -> Self {
        self.current_color = color;
        self
    }

    pub fn pending_color(mut self, color: gpui::Hsla) -> Self {
        self.pending_color = color;
        self
    }
}

impl RenderOnce for ProgressStepper {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let progress = if self.total_steps > 0 {
            (self.current_step as f32 / self.total_steps as f32 * 100.0).min(100.0)
        } else {
            0.0
        };

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap_2()
            .w_full()
            .child(
                // Progress bar
                div()
                    .h(px(4.0))
                    .w_full()
                    .bg(self.pending_color)
                    .rounded_full()
                    .overflow_hidden()
                    .child(
                        div()
                            .h_full()
                            .w(pct(progress))
                            .bg(self.current_color)
                            .rounded_full(),
                    ),
            )
            .child(
                // Step indicators
                div()
                    .flex()
                    .justify_between()
                    .children((0..self.total_steps).map(|i| {
                        let color = if i < self.current_step {
                            self.completed_color
                        } else if i == self.current_step {
                            self.current_color
                        } else {
                            self.pending_color
                        };

                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_1()
                            .child(
                                div()
                                    .size(px(8.0))
                                    .rounded_full()
                                    .bg(color),
                            )
                            .when(self.show_labels && i < self.labels.len(), |d| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .text_color(rgba(0x888888ff))
                                        .child(self.labels[i].clone()),
                                )
                            })
                    })),
            )
    }
}
