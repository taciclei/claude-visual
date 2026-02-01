//! Log entry rendering

use gpui::*;
use gpui::prelude::*;

use crate::app::theme::Theme;
use crate::ui::mcp::logs::core::McpLogsPanel;
use crate::ui::mcp::logs::types::{LogEntry, McpLogsPanelEvent};

impl McpLogsPanel {
    /// Render a log entry
    pub(super) fn render_log_entry(
        &self,
        index: usize,
        entry: &LogEntry,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_selected = self.selected == Some(index);
        let level_color = entry.level.color();
        let accent_color = theme.colors.accent;
        let surface_hover = theme.colors.surface_hover;
        let text_muted = theme.colors.text_muted;
        let text_color = theme.colors.text;

        let on_click_handler = cx.listener(move |this, _, _window, cx| {
            this.selected = Some(index);
            cx.emit(McpLogsPanelEvent::LogClicked(index));
            cx.notify();
        });

        div()
            .id(ElementId::Name(format!("log-entry-{}", index).into()))
            .w_full()
            .px_3()
            .py_1()
            .flex()
            .items_start()
            .gap_2()
            .cursor_pointer()
            .bg(if is_selected {
                accent_color.opacity(0.15)
            } else {
                gpui::transparent_black()
            })
            .hover(|s| s.bg(surface_hover))
            .on_click(on_click_handler)
            // Timestamp
            .child(
                div()
                    .w(px(60.0))
                    .flex_shrink_0()
                    .text_xs()
                    .text_color(text_muted)
                    .child(entry.elapsed_str()),
            )
            // Level badge
            .child(
                div()
                    .w(px(50.0))
                    .flex_shrink_0()
                    .px_1()
                    .py_px()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(level_color)
                    .child(entry.level.label()),
            )
            // Server
            .child(
                div()
                    .w(px(80.0))
                    .flex_shrink_0()
                    .text_xs()
                    .text_color(accent_color)
                    .text_ellipsis()
                    .child(entry.server.clone()),
            )
            // Message
            .child(
                div()
                    .flex_1()
                    .text_xs()
                    .text_color(text_color)
                    .overflow_hidden()
                    .text_ellipsis()
                    .child(entry.message.clone()),
            )
            // Context (if any)
            .when_some(entry.context.clone(), |d, context| {
                d.child(
                    div()
                        .max_w(px(150.0))
                        .text_xs()
                        .text_color(text_muted)
                        .text_ellipsis()
                        .child(context),
                )
            })
    }
}
