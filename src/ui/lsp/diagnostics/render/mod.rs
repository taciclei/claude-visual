//! Rendering implementation for diagnostics panel

use gpui::*;
use gpui::prelude::*;

use super::core::DiagnosticsPanel;
use super::types::{default_colors, DiagnosticsPanelEvent, SimpleColors};

mod header;
mod file_list;
mod diagnostic_item;
mod empty_state;

use header::render_header;
use file_list::render_file_list;

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
