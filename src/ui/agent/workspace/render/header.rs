//! Header section rendering

use gpui::*;
use gpui::prelude::*;
use super::super::types::*;
use super::super::helpers::{mode_color, mode_label};

pub(crate) fn render_header(
    mode: AgentMode,
    task_description: Option<String>,
    colors: &SimpleColors,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap_2()
        // Mode indicator
        .child(
            div()
                .w(px(8.0))
                .h(px(8.0))
                .rounded_full()
                .bg(mode_color(mode, colors)),
        )
        // Mode label
        .child(
            div()
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .text_color(colors.text)
                .child(mode_label(mode)),
        )
        // Task description
        .when(task_description.is_some(), |d| {
            d.child(
                div()
                    .text_sm()
                    .text_color(colors.text_muted)
                    .max_w(px(300.0))
                    .text_ellipsis()
                    .overflow_hidden()
                    .child(format!("- {}", task_description.unwrap())),
            )
        })
}
