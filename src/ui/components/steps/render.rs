use gpui::prelude::*;
use gpui::*;

use super::{StepStatus, Steps};

impl Steps {
    pub(super) fn status_color(&self, status: StepStatus) -> gpui::Hsla {
        match status {
            StepStatus::Completed => self.completed_color,
            StepStatus::Current => self.current_color,
            StepStatus::Error => self.error_color,
            StepStatus::Pending | StepStatus::Skipped => self.pending_color,
        }
    }

    pub(super) fn render_indicator(&self, index: usize, status: StepStatus) -> impl IntoElement {
        let indicator_size = self.size.indicator_size();
        let font_size = self.size.font_size();
        let color = self.status_color(status);

        let content: Box<dyn FnOnce() -> String> = if status == StepStatus::Completed {
            Box::new(|| "✓".to_string())
        } else if status == StepStatus::Error {
            Box::new(|| "✕".to_string())
        } else if self.show_numbers {
            let num = index + 1;
            Box::new(move || format!("{}", num))
        } else {
            Box::new(|| "".to_string())
        };

        match self.variant {
            super::StepsVariant::Circle | super::StepsVariant::CircleAlt => div()
                .size(px(indicator_size))
                .rounded_full()
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(font_size))
                .when(
                    status == StepStatus::Current || status == StepStatus::Completed,
                    |d| d.bg(color).text_color(rgb(0xffffff)),
                )
                .when(
                    status == StepStatus::Pending || status == StepStatus::Skipped,
                    |d| d.border_2().border_color(color).text_color(color),
                )
                .when(status == StepStatus::Error, |d| {
                    d.bg(color).text_color(rgb(0xffffff))
                })
                .child(content()),
            super::StepsVariant::Dot => div()
                .size(px(indicator_size * 0.4))
                .rounded_full()
                .bg(color),
            super::StepsVariant::Line => div()
                .w(px(indicator_size))
                .h(px(4.0))
                .rounded_full()
                .bg(color),
            super::StepsVariant::Icon => div()
                .size(px(indicator_size))
                .rounded_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(color)
                .text_color(rgb(0xffffff))
                .text_size(px(font_size))
                .child(content()),
        }
    }

    pub(super) fn render_connector(&self, completed: bool) -> impl IntoElement {
        let thickness = self.size.connector_thickness();
        let color = if completed {
            self.completed_color
        } else {
            self.connector_color
        };

        match self.orientation {
            super::StepsOrientation::Horizontal => div().h(px(thickness)).flex_1().mx_2().bg(color),
            super::StepsOrientation::Vertical => {
                div().w(px(thickness)).h(px(20.0)).my_1().bg(color)
            }
        }
    }
}

impl RenderOnce for Steps {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // Copy theme colors for move closures
        let text_color: Hsla = rgba(0xccccccff).into();
        let desc_color: Hsla = rgba(0x888888ff).into();
        let font_size = self.size.font_size();

        let id = self.id.clone();
        let base = div().id(id);

        match self.orientation {
            super::StepsOrientation::Horizontal => base.flex().items_center().w_full().children(
                self.steps.iter().enumerate().flat_map(|(i, step)| {
                    let status = if i < self.current {
                        StepStatus::Completed
                    } else if i == self.current {
                        step.status
                    } else {
                        StepStatus::Pending
                    };

                    let mut elements: Vec<Box<dyn FnOnce() -> gpui::AnyElement>> =
                        vec![Box::new({
                            let indicator = self.render_indicator(i, status);
                            let title = step.title.clone();
                            let description = step.description.clone();
                            let clickable = self.clickable;
                            move || {
                                div()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .gap_1()
                                    .when(clickable, |d| d.cursor_pointer())
                                    .child(indicator)
                                    .child(
                                        div()
                                            .text_size(px(font_size))
                                            .text_color(text_color)
                                            .font_weight(if status == StepStatus::Current {
                                                gpui::FontWeight::SEMIBOLD
                                            } else {
                                                gpui::FontWeight::NORMAL
                                            })
                                            .child(title),
                                    )
                                    .when_some(description, |d, desc| {
                                        d.child(
                                            div()
                                                .text_size(px(font_size - 2.0))
                                                .text_color(desc_color)
                                                .child(desc),
                                        )
                                    })
                                    .into_any_element()
                            }
                        })];

                    if i < self.steps.len() - 1 {
                        let connector = self.render_connector(i < self.current);
                        elements.push(Box::new(move || connector.into_any_element()));
                    }

                    elements.into_iter().map(|f| f())
                }),
            ),

            super::StepsOrientation::Vertical => {
                base.flex()
                    .flex_col()
                    .children(self.steps.iter().enumerate().flat_map(|(i, step)| {
                        let status = if i < self.current {
                            StepStatus::Completed
                        } else if i == self.current {
                            step.status
                        } else {
                            StepStatus::Pending
                        };

                        let mut elements: Vec<Box<dyn FnOnce() -> gpui::AnyElement>> =
                            vec![Box::new({
                                let indicator = self.render_indicator(i, status);
                                let title = step.title.clone();
                                let description = step.description.clone();
                                let clickable = self.clickable;
                                move || {
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_3()
                                        .when(clickable, |d| d.cursor_pointer())
                                        .child(indicator)
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .child(
                                                    div()
                                                        .text_size(px(font_size))
                                                        .text_color(text_color)
                                                        .font_weight(
                                                            if status == StepStatus::Current {
                                                                gpui::FontWeight::SEMIBOLD
                                                            } else {
                                                                gpui::FontWeight::NORMAL
                                                            },
                                                        )
                                                        .child(title),
                                                )
                                                .when_some(description, |d, desc| {
                                                    d.child(
                                                        div()
                                                            .text_size(px(font_size - 2.0))
                                                            .text_color(desc_color)
                                                            .child(desc),
                                                    )
                                                }),
                                        )
                                        .into_any_element()
                                }
                            })];

                        if i < self.steps.len() - 1 {
                            let connector = self.render_connector(i < self.current);
                            let indicator_size = self.size.indicator_size();
                            elements.push(Box::new(move || {
                                div()
                                    .flex()
                                    .ml(px(indicator_size / 2.0 - 1.0))
                                    .child(connector)
                                    .into_any_element()
                            }));
                        }

                        elements.into_iter().map(|f| f())
                    }))
            }
        }
    }
}
