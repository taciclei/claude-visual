//! Git panel rendering orchestration

mod header;
mod content;
mod actions;

pub use header::*;
pub use content::*;
pub use actions::*;

use gpui::*;
use gpui::prelude::*;

use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Render git panel modal
    pub fn render_git_panel(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> impl IntoElement {
        // Extract listener before div chain
        let toggle_overlay = cx.listener(|this, _, _window, cx| {
            this.toggle_git_panel(cx);
        });

        div()
            .id("git-panel-overlay")
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .on_click(toggle_overlay)
            .child(
                div()
                    .id("git-panel")
                    .w(px(400.0))
                    .bg(theme.colors.surface)
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.colors.border)
                    .shadow_lg()
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    .on_click(|_, _, _| {})
                    .child(self.render_git_panel_header(theme, cx))
                    .child(self.render_git_panel_content(theme))
                    .when_some(self.render_git_panel_actions(theme, cx), |d, actions| {
                        d.child(actions)
                    })
            )
    }
}
