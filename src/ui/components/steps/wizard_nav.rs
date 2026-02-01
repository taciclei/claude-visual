use gpui::*;
use gpui::prelude::*;

/// Wizard navigation component
#[derive(IntoElement)]
pub struct WizardNav {
    id: ElementId,
    current_step: usize,
    total_steps: usize,
    can_go_back: bool,
    can_go_next: bool,
    back_label: SharedString,
    next_label: SharedString,
    finish_label: SharedString,
    show_step_count: bool,
}

impl WizardNav {
    pub fn new(id: impl Into<ElementId>, current: usize, total: usize) -> Self {
        Self {
            id: id.into(),
            current_step: current,
            total_steps: total,
            can_go_back: current > 0,
            can_go_next: true,
            back_label: "Back".into(),
            next_label: "Next".into(),
            finish_label: "Finish".into(),
            show_step_count: true,
        }
    }

    pub fn can_go_back(mut self, can: bool) -> Self {
        self.can_go_back = can;
        self
    }

    pub fn can_go_next(mut self, can: bool) -> Self {
        self.can_go_next = can;
        self
    }

    pub fn back_label(mut self, label: impl Into<SharedString>) -> Self {
        self.back_label = label.into();
        self
    }

    pub fn next_label(mut self, label: impl Into<SharedString>) -> Self {
        self.next_label = label.into();
        self
    }

    pub fn finish_label(mut self, label: impl Into<SharedString>) -> Self {
        self.finish_label = label.into();
        self
    }

    pub fn show_step_count(mut self, show: bool) -> Self {
        self.show_step_count = show;
        self
    }
}

impl RenderOnce for WizardNav {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let is_last = self.current_step >= self.total_steps - 1;

        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .py_4()
            .border_t_1()
            .border_color(rgba(0x8888881a))
            .child(
                // Back button
                div()
                    .px_4()
                    .py_2()
                    .rounded_md()
                    .cursor_pointer()
                    .when(self.can_go_back && self.current_step > 0, |d| {
                        d.bg(rgba(0x8888881a))
                            .text_color(rgba(0xccccccff))
                    })
                    .when(!self.can_go_back || self.current_step == 0, |d| {
                        d.opacity(0.5).cursor_default()
                    })
                    .child(self.back_label.clone()),
            )
            .when(self.show_step_count, |d| {
                d.child(
                    div()
                        .text_sm()
                        .text_color(rgba(0x888888ff))
                        .child(format!("Step {} of {}", self.current_step + 1, self.total_steps)),
                )
            })
            .child(
                // Next/Finish button
                div()
                    .px_4()
                    .py_2()
                    .rounded_md()
                    .cursor_pointer()
                    .when(self.can_go_next, |d| {
                        d.bg(rgb(0x3b82f6)).text_color(rgb(0xffffff))
                    })
                    .when(!self.can_go_next, |d| {
                        d.bg(rgba(0x3b82f64d))
                            .text_color(rgba(0xffffff80))
                            .cursor_default()
                    })
                    .child(if is_last {
                        self.finish_label.clone()
                    } else {
                        self.next_label.clone()
                    }),
            )
    }
}
