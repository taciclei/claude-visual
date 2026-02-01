//! Simple progress steps display

use gpui::*;
use gpui::prelude::*;

/// Simple progress steps display
#[derive(Clone)]
pub struct ProgressSteps {
    steps: Vec<String>,
    current: usize,
}

impl ProgressSteps {
    pub fn new(steps: Vec<impl Into<String>>, current: usize) -> Self {
        Self {
            steps: steps.into_iter().map(|s| s.into()).collect(),
            current,
        }
    }
}

impl RenderOnce for ProgressSteps {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let muted = hsla(0.0, 0.0, 0.5, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);
        let success = hsla(0.38, 0.7, 0.45, 1.0);
        let border = hsla(0.0, 0.0, 0.25, 1.0);

        div()
            .w_full()
            .flex()
            .items_center()
            .children(self.steps.into_iter().enumerate().map(|(idx, label)| {
                let is_last = idx == 0; // Would need length
                let is_completed = idx < self.current;
                let is_active = idx == self.current;

                let (dot_color, label_color) = if is_completed {
                    (success, muted)
                } else if is_active {
                    (accent, text)
                } else {
                    (border, muted)
                };

                div()
                    .flex()
                    .items_center()
                    .flex_1()
                    // Step
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_1()
                            // Dot
                            .child(
                                div()
                                    .size(px(24.0))
                                    .rounded_full()
                                    .bg(dot_color)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_xs()
                                    .text_color(gpui::white())
                                    .child(if is_completed { "✓".to_string() } else { (idx + 1).to_string() })
                            )
                            // Label
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(label_color)
                                    .text_center()
                                    .child(label)
                            )
                    )
                    // Connector
                    .child(
                        div()
                            .flex_1()
                            .h(px(2.0))
                            .mx_2()
                            .bg(if is_completed { success } else { border })
                    )
            }))
    }
}
