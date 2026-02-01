//! Rendering for MCP servers panel

use gpui::*;
use gpui::prelude::*;
use super::core::McpServersPanel;

impl Render for McpServersPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();

        div()
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .w_full()
            .bg(theme.colors.surface)
            .rounded_lg()
            .border_1()
            .border_color(theme.colors.border)
            .child(self.render_header(cx))
            .when(self.expanded, |this| {
                let servers: Vec<_> = self.servers.clone();
                this.child(
                    div()
                        .flex()
                        .flex_col()
                        .p_2()
                        .gap_1()
                        .when(servers.is_empty(), |this| {
                this.child(self.render_empty(cx))
                        })
                        .children(
                            servers
                                .iter()
                                .enumerate()
                                .map(|(i, server)| self.render_server_item(server, i, cx)),
                        ),
                )
            })
    }
}
