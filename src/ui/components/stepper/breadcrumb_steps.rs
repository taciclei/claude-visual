//! Breadcrumb-style step indicator

use gpui::*;
use gpui::prelude::*;

/// Breadcrumb-style step indicator
#[derive(Clone)]
pub struct BreadcrumbSteps {
    steps: Vec<String>,
    current: usize,
}

impl BreadcrumbSteps {
    pub fn new(steps: Vec<impl Into<String>>, current: usize) -> Self {
        Self {
            steps: steps.into_iter().map(|s| s.into()).collect(),
            current,
        }
    }
}

impl RenderOnce for BreadcrumbSteps {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let text = hsla(0.0, 0.0, 0.9, 1.0);
        let muted = hsla(0.0, 0.0, 0.5, 1.0);
        let accent = hsla(0.6, 0.8, 0.6, 1.0);

        div()
            .flex()
            .items_center()
            .gap_2()
            .children(self.steps.into_iter().enumerate().map(|(idx, label)| {
                let is_active = idx == self.current;
                let is_completed = idx < self.current;
                let is_last = idx == 0; // Would need length

                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Step text
                    .child(
                        div()
                            .text_sm()
                            .when(is_active, |d| {
                                d.font_weight(FontWeight::SEMIBOLD)
                                    .text_color(accent)
                            })
                            .when(is_completed, |d| {
                                d.text_color(text)
                                    .cursor_pointer()
                            })
                            .when(!is_active && !is_completed, |d| {
                                d.text_color(muted)
                            })
                            .child(label)
                    )
                    // Separator
                    .child(
                        div()
                            .text_color(muted)
                            .text_sm()
                            .child("›")
                    )
            }))
    }
}
