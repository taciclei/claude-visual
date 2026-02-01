use gpui::prelude::*;
use gpui::*;

/// Numbered step list
#[derive(IntoElement)]
pub struct NumberedSteps {
    id: ElementId,
    steps: Vec<SharedString>,
    completed: Vec<bool>,
    text_color: gpui::Hsla,
    number_color: gpui::Hsla,
    completed_color: gpui::Hsla,
}

impl NumberedSteps {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            steps: Vec::new(),
            completed: Vec::new(),
            text_color: rgba(0xccccccff).into(),
            number_color: rgb(0x3b82f6).into(),
            completed_color: rgb(0x22c55e).into(),
        }
    }

    pub fn steps(mut self, steps: Vec<impl Into<SharedString>>) -> Self {
        self.steps = steps.into_iter().map(|s| s.into()).collect();
        self.completed = vec![false; self.steps.len()];
        self
    }

    pub fn completed(mut self, completed: Vec<bool>) -> Self {
        self.completed = completed;
        self
    }

    pub fn mark_completed(mut self, index: usize) -> Self {
        if index < self.completed.len() {
            self.completed[index] = true;
        }
        self
    }

    pub fn text_color(mut self, color: gpui::Hsla) -> Self {
        self.text_color = color;
        self
    }

    pub fn number_color(mut self, color: gpui::Hsla) -> Self {
        self.number_color = color;
        self
    }

    pub fn completed_color(mut self, color: gpui::Hsla) -> Self {
        self.completed_color = color;
        self
    }
}

impl RenderOnce for NumberedSteps {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap_3()
            .children(self.steps.iter().enumerate().map(|(i, step)| {
                let is_completed = self.completed.get(i).copied().unwrap_or(false);
                let color = if is_completed {
                    self.completed_color
                } else {
                    self.number_color
                };

                div()
                    .flex()
                    .items_start()
                    .gap_3()
                    .child(
                        div()
                            .size(px(24.0))
                            .rounded_full()
                            .bg(color.opacity(0.1))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(color)
                            .child(if is_completed {
                                "✓".to_string()
                            } else {
                                format!("{}", i + 1)
                            }),
                    )
                    .child(
                        div()
                            .flex_1()
                            .pt(px(2.0))
                            .text_color(self.text_color)
                            .when(is_completed, |d| d.line_through())
                            .child(step.clone()),
                    )
            }))
    }
}
