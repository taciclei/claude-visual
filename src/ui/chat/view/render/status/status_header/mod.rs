//! Status header rendering for ChatView

mod connection;
mod helpers;
mod left_section;
mod right_section;

use super::super::super::core::ChatView;
use connection::render_connection_status;
use gpui::prelude::*;
use gpui::*;

impl ChatView {
    pub fn render_status_header(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Div {
        div()
            .w_full()
            .flex()
            .items_center()
            .justify_between()
            .px_4()
            .py_2()
            .bg(theme.colors.surface)
            .border_b_1()
            .border_color(theme.colors.border)
            // Left side - Connection status and badges
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    // Status indicator
                    .child(render_connection_status(&self.connection_status, theme))
                    // Model info (clickable for model switching)
                    .when_some(self.session_info.as_ref(), |d, info| {
                        d.child(self.render_model_badge(info, theme, cx))
                    })
                    // Tools count (when available) - clickable to show session details
                    .when_some(
                        self.session_info.as_ref().filter(|i| !i.tools.is_empty()),
                        |d, info| d.child(self.render_tools_badge(info, theme, cx)),
                    )
                    // Skills/commands count (when available) - clickable to show session details
                    .when_some(
                        self.session_info
                            .as_ref()
                            .filter(|i| !i.skills.is_empty() || !i.slash_commands.is_empty()),
                        |d, info| d.child(self.render_skills_badge(info, theme, cx)),
                    )
                    // Agents count (when available) - clickable to show session details
                    .when_some(
                        self.session_info.as_ref().filter(|i| !i.agents.is_empty()),
                        |d, info| d.child(self.render_agents_badge(info, theme, cx)),
                    )
                    // Think mode indicator (when enabled)
                    .when(self.think_mode_enabled, |d| {
                        d.child(self.render_think_mode_badge(theme, cx))
                    })
                    // Pending permissions indicator (when there are requests)
                    .when(!self.pending_permissions.is_empty(), |d| {
                        let count = self.pending_permissions.len();
                        d.child(self.render_permissions_badge(count, theme, cx))
                    })
                    // Working directory (when available) - Click to reveal in Finder
                    .when_some(
                        self.session_info
                            .as_ref()
                            .filter(|i| !i.cwd.is_empty())
                            .cloned(),
                        |d, info| d.child(self.render_cwd_badge(&info, theme)),
                    )
                    // MCP servers status (when available)
                    .when_some(
                        self.session_info
                            .as_ref()
                            .filter(|i| !i.mcp_servers.is_empty()),
                        |d, info| d.child(self.render_mcp_badge(info, theme)),
                    )
                    // Session ID badge (when available) - Click to copy full ID
                    .when_some(
                        self.session_info
                            .as_ref()
                            .map(|i| i.session_id.clone())
                            .filter(|id| !id.is_empty()),
                        |d, session_id| {
                            d.child(self.render_session_id_badge_left(session_id, theme, cx))
                        },
                    ),
            )
            // Right side - Active tasks and indicators
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Active tasks (up to 3)
                    .children(
                        self.active_tasks
                            .iter()
                            .enumerate()
                            .take(3)
                            .map(|(idx, task)| self.render_active_task(idx, task, theme, cx)),
                    )
                    // Show count if more tasks
                    .when(self.active_tasks.len() > 3, |d| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .child(format!("+{} more", self.active_tasks.len() - 3)),
                        )
                    })
                    // Session ID badge (when available and no active tasks)
                    .when_some(
                        self.session_info
                            .as_ref()
                            .filter(|i| !i.session_id.is_empty() && self.active_tasks.is_empty()),
                        |d, info| d.child(self.render_session_id_badge_right(info, theme)),
                    )
                    // Version badge (when no active tasks and version is available)
                    .when_some(
                        self.session_info
                            .as_ref()
                            .filter(|i| !i.version.is_empty() && self.active_tasks.is_empty()),
                        |d, info| d.child(self.render_version_badge(info, theme)),
                    )
                    // Session health indicator (always show when session is active)
                    .when(self.session_info.is_some(), |d| {
                        d.child(self.render_session_health_indicator(&theme, cx))
                    })
                    // Latency indicator (when we have latency data)
                    .when(self.stats.last_response_latency_ms.is_some(), |d| {
                        d.child(self.render_latency_indicator(&theme))
                    })
                    // Session cost indicator (when we have cost data)
                    .when(self.stats.cost > 0.0, |d| {
                        let cost = self.stats.cost;
                        d.child(self.render_cost_indicator(cost, theme, cx))
                    })
                    // Help button (keyboard shortcuts)
                    .child(
                        div()
                            .id("help-button")
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .hover(|s| {
                                s.bg(theme.colors.surface_hover)
                                    .text_color(theme.colors.text)
                            })
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.toggle_shortcuts_help(cx);
                            }))
                            .child("?"),
                    ),
            )
    }

    // render_model_switcher moved to models.rs
}
