//! Vertical stepper layout

use gpui::*;
use gpui::prelude::*;
use super::types::*;
use super::component::Stepper;

pub fn render_vertical_step(
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
        StepStatus::Completed => (completed_color.opacity(0.15), completed_color, completed_color),
        StepStatus::Active => (active_color.opacity(0.15), active_color, active_color),
        StepStatus::Error => (error_color.opacity(0.15), error_color, error_color),
        StepStatus::Skipped => (pending_color.opacity(0.1), pending_color, pending_color),
        StepStatus::Pending => (surface_color.opacity(0.0), pending_color, border_color_default),
    };

    let step_icon = step.icon.clone().unwrap_or_else(|| {
        match step.status {
            StepStatus::Completed => "✓".to_string(),
            StepStatus::Error => "✕".to_string(),
            StepStatus::Skipped => "−".to_string(),
            _ if show_numbers => (index + 1).to_string(),
            _ => "○".to_string(),
        }
    });

    let step_label = step.label.clone();
    let step_description = step.description.clone();
    let step_optional = step.optional;

    let on_click_circle = cx.listener(move |this, _, _window, cx| {
        this.go_to(index, cx);
    });

    let on_click_content = cx.listener(move |this, _, _window, cx| {
        this.go_to(index, cx);
    });

    div()
        .id(SharedString::from(format!("step-{}", index)))
        .w_full()
        .flex()
        .child(
            div()
                .w(px(48.0))
                .flex()
                .flex_col()
                .items_center()
                .child(
                    div()
                        .id(SharedString::from(format!("step-v-circle-{}", index)))
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
                        .when(is_clickable, |d| d.cursor_pointer().on_click(on_click_circle))
                        .child(step_icon)
                )
                .when(!is_last, |d| {
                    d.child(
                        div()
                            .w(px(2.0))
                            .flex_1()
                            .min_h(px(24.0))
                            .bg(if index < current {
                                completed_color
                            } else {
                                border_color_default
                            })
                    )
                })
        )
        .child(
            div()
                .id(SharedString::from(format!("step-v-content-{}", index)))
                .flex_1()
                .pb_4()
                .flex()
                .flex_col()
                .gap_1()
                .when(is_clickable, |d| d.cursor_pointer().on_click(on_click_content))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(text_color)
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(step_label)
                        .when(step_optional, |d| {
                            d.child(
                                div()
                                    .text_xs()
                                    .text_color(text_muted)
                                    .child("(Optional)")
                            )
                        })
                )
                .when_some(step_description, |d, desc| {
                    d.child(
                        div()
                            .text_xs()
                            .text_color(text_muted)
                            .child(desc)
                    )
                })
        )
}
