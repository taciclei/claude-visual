//! Active tasks button

use crate::ui::chat::view::core::ChatView;
use gpui::prelude::*;
use gpui::*;

impl ChatView {
    /// Render active tasks panel button (⚡ with count)
    pub(in crate::ui::chat::view::render::toolbar::quick_actions_bar) fn render_tasks_panel_button(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let task_count = self.active_task_count();
        let text_color_active = theme.colors.accent;
        let text_color_inactive = theme.colors.text_muted;
        let text_color_hover = theme.colors.text;
        let text_color_available = theme.colors.warning;
        let warning_bg = theme.colors.warning.opacity(0.1);
        let surface_hover = theme.colors.surface_hover;

        let on_click = cx.listener(|this, _, _window, cx| {
            this.toggle_tasks_panel(cx);
        });

        div()
            .id("tasks-panel-btn")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py(px(2.0))
            .rounded_md()
            .cursor_pointer()
            .text_xs()
            .text_color(if self.show_tasks_panel {
                text_color_active
            } else if task_count > 0 {
                text_color_available
            } else {
                text_color_inactive
            })
            .when(task_count > 0, move |d| d.bg(warning_bg))
            .hover(move |s| s.bg(surface_hover).text_color(text_color_hover))
            .on_click(on_click)
            .child("⚡")
            .when(task_count > 0, |d| d.child(format!("{} tasks", task_count)))
            .when(task_count == 0, |d| d.child("Tasks"))
    }
}
