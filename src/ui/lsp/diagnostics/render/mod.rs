//! Rendering implementation for diagnostics panel

use gpui::prelude::*;
use gpui::*;

use super::core::DiagnosticsPanel;
use super::types::{default_colors, DiagnosticsPanelEvent, SimpleColors};

mod diagnostic_item;
mod empty_state;
mod file_list;
mod header;

use file_list::render_file_list;
use header::render_header;

impl EventEmitter<DiagnosticsPanelEvent> for DiagnosticsPanel {}

impl Render for DiagnosticsPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = default_colors();

        div()
            .flex()
            .flex_col()
            .w_full()
            .bg(colors.panel)
            .border_t_1()
            .border_color(colors.border)
            .child(render_header(self, cx, &colors))
            .when(self.is_expanded, |this| {
                this.child(render_file_list(self, cx, &colors))
            })
    }
}
