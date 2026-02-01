//! Tab button rendering

use gpui::*;
use gpui::prelude::*;

use super::super::{TeamPanel, TeamViewMode};

impl TeamPanel {
    /// Render tab button
    pub(super) fn render_tab_button(&self, label: &str, mode: TeamViewMode, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let is_active = self.view_mode == mode;
        let label = label.to_string();

        div()
            .id(ElementId::Name(format!("tab-{:?}", mode).into()))
            .px_3()
            .py_1()
            .rounded_md()
            .bg(if is_active { theme.colors.accent } else { theme.colors.surface })
            .hover(|s| {
                s.bg(if is_active {
                    theme.colors.accent_hover
                } else {
                    theme.colors.surface_hover
                })
            })
            .cursor_pointer()
            .text_sm()
            .text_color(theme.colors.text)
            .on_click(cx.listener(move |this, _, _window, cx| {
                this.set_view_mode(mode, cx);
            }))
            .child(label)
    }
}
