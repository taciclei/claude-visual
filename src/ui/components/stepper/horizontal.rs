//! Horizontal stepper layout

use super::component::Stepper;
use super::types::*;
use gpui::prelude::*;
use gpui::*;

pub fn render_horizontal_step(
    index: usize,
    step: &Step,
    is_last: bool,
    is_clickable: bool,
    current: usize,
    show_numbers: bool,
    completed_color: Hsla,
    active_color: Hsla,
    pending_color: Hsla,
    error_color: Hsla,
    border_color_default: Hsla,
    surface_color: Hsla,
    text_muted: Hsla,
    cx: &mut Context<Stepper>,
) -> impl IntoElement {
    let (bg_color, text_color, border_color) = match step.status {
        StepStatus::Completed => (
            completed_color.opacity(0.15),
            completed_color,
            completed_color,
        ),
        StepStatus::Active => (active_color.opacity(0.15), active_color, active_color),
        StepStatus::Error => (error_color.opacity(0.15), error_color, error_color),
        StepStatus::Skipped => (pending_color.opacity(0.1), pending_color, pending_color),
        StepStatus::Pending => (
            surface_color.opacity(0.0),
            pending_color,
            border_color_default,
        ),
    };

    let step_icon = step.icon.clone().unwrap_or_else(|| match step.status {
        StepStatus::Completed => "✓".to_string(),
        StepStatus::Error => "✕".to_string(),
        StepStatus::Skipped => "−".to_string(),
        _ if show_numbers => (index + 1).to_string(),
        _ => "○".to_string(),
    });

    let step_label = step.label.clone();
    let step_optional = step.optional;

    let on_click = cx.listener(move |this, _, _window, cx| {
        this.go_to(index, cx);
    });

    div()
        .id(SharedString::from(format!("step-{}", index)))
        .flex()
        .items_center()
        .flex_1()
        .child(
            div()
                .id(SharedString::from(format!("step-h-{}", index)))
                .flex()
                .flex_col()
                .items_center()
                .gap_1()
                .when(is_clickable, |d| d.cursor_pointer().on_click(on_click))
                .child(
                    div()
                        .size(px(32.0))
                        .rounded_full()
                        .border_2()
                        .border_color(border_color)
                        .bg(bg_color)
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(text_color)
                        .child(step_icon),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(text_color)
                        .text_center()
                        .max_w(px(80.0))
                        .child(step_label),
                )
                .when(step_optional, |d| {
                    d.child(div().text_xs().text_color(text_muted).child("(Optional)"))
                }),
        )
        .when(!is_last, |d| {
            d.child(div().flex_1().h(px(2.0)).mx_2().bg(if index < current {
                completed_color
            } else {
                border_color_default
            }))
        })
}
