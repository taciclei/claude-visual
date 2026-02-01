//! Permissions indicator button

use crate::ui::chat::view::core::ChatView;
use gpui::*;

impl ChatView {
    /// Render permissions indicator button (only when pending permissions exist)
    pub(in crate::ui::chat::view::render::toolbar::quick_actions_bar) fn render_permissions_button(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Option<Stateful<Div>> {
        if self.pending_permissions.is_empty() {
            return None;
        }

        let warning_bg = theme.colors.warning.opacity(0.1);
        let warning_hover = theme.colors.warning.opacity(0.2);
        let warning_color = theme.colors.warning;
        let count = self.pending_permissions.len();

        let on_click = cx.listener(|this, _, _window, cx| {
            this.toggle_permissions_panel(cx);
        });

        Some(
            div()
                .id("permissions-btn")
                .flex()
                .items_center()
                .gap_1()
                .px_2()
                .py(px(2.0))
                .rounded_md()
                .cursor_pointer()
                .text_xs()
                .bg(warning_bg)
                .text_color(warning_color)
                .hover(move |s| s.bg(warning_hover))
                .on_click(on_click)
                .child("🔐")
                .child(format!("{} pending", count)),
        )
    }
}
