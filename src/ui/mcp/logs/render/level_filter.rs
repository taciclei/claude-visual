//! Level filter button rendering

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;
use crate::ui::mcp::logs::core::McpLogsPanel;
use crate::ui::mcp::logs::types::LogLevel;

impl McpLogsPanel {
    /// Render level filter button
    pub(super) fn render_level_filter(
        &self,
        level: LogLevel,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_active = self.filter.min_level == level;
        let level_color = level.color();
        let text_muted = theme.colors.text_muted;

        let on_click_handler = cx.listener(move |this, _, _window, cx| {
            this.set_min_level(level, cx);
        });

        div()
            .id(ElementId::Name(format!("level-filter-{}", level.label()).into()))
            .px_2()
            .py_1()
            .text_xs()
            .rounded_sm()
            .cursor_pointer()
            .bg(if is_active {
                level_color.opacity(0.2)
            } else {
                gpui::transparent_black()
            })
            .text_color(if is_active {
                level_color
            } else {
                text_muted
            })
            .hover(|s| s.bg(level_color.opacity(0.1)))
            .on_click(on_click_handler)
            .child(level.label())
    }
}
