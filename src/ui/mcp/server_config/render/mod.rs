//! Rendering implementation for MCP server configuration editor

use gpui::prelude::*;
use gpui::*;

use super::core::ServerConfigEditor;

mod content;
mod field;
mod footer;
mod header;

pub(crate) use content::*;
pub(crate) use field::*;
pub(crate) use footer::*;
pub(crate) use header::*;

impl Render for ServerConfigEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_clone = {
            let theme = self.app_state.theme.read(cx);
            theme.clone()
        };
        let theme = &theme_clone;

        div()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .bg(theme.colors.background)
            .child(self.render_header(theme, cx))
            .child(self.render_content(theme, cx))
            .child(self.render_footer(theme, cx))
    }
}
