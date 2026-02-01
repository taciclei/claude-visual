//! Debug status indicator

use gpui::*;
use gpui::prelude::*;

use crate::ui::debug::debug_panel::DebugPanel;
use crate::debug::DebugState;

impl DebugPanel {
    /// Render status indicator
    pub(super) fn render_status(
        &self,
        state: DebugState,
        text_muted_color: Hsla,
    ) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_2()
            // State indicator
            .child(
                div()
                    .w(px(8.0))
                    .h(px(8.0))
                    .rounded_full()
                    .bg({
                        let (r, g, b) = state.color();
                        rgb(((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
                    }),
            )
            // State text
            .child(
                div()
                    .text_xs()
                    .text_color(text_muted_color)
                    .child(format!("{:?}", state)),
            )
    }

    /// Render separator
    pub(super) fn render_separator(border_color: Hsla) -> impl IntoElement {
        div()
            .w(px(1.0))
            .h(px(20.0))
            .bg(border_color)
            .mx_1()
    }
}
