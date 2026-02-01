//! Empty state rendering for diagnostics panel

use gpui::prelude::*;
use gpui::*;

use crate::ui::lsp::diagnostics::types::SimpleColors;

pub fn render_empty_state(colors: &SimpleColors) -> impl IntoElement {
    let colors_text_muted = colors.text_muted;

    div().flex().items_center().justify_center().py_4().child(
        div()
            .text_sm()
            .text_color(colors_text_muted)
            .child("No problems detected"),
    )
}
