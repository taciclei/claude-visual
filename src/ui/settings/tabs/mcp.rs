use super::super::SettingsModal;
use gpui::prelude::*;
use gpui::*;

impl SettingsModal {
    /// Render the MCP tab
    pub(crate) fn render_mcp_tab(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let mcp_config = crate::mcp::McpConfig::load_default(None).unwrap_or_default();
        let server_count = mcp_config.mcp_servers.len();
        let enabled_count = mcp_config.enabled_servers().count();

        div()
            .flex()
            .flex_col()
            .gap_4()
            // Overview
            .child(
                self.render_section(
                    "MCP Servers",
                    "Model Context Protocol servers extend Claude's capabilities",
                    div()
                        .flex()
                        .flex_col()
                        .gap_3()
                        // Stats
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_4()
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .child(
                                            div()
                                                .text_2xl()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.colors.text)
                                                .child(format!("{}", server_count)),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.text_muted)
                                                .child("Configured"),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .child(
                                            div()
                                                .text_2xl()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.colors.success)
                                                .child(format!("{}", enabled_count)),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.colors.text_muted)
                                                .child("Enabled"),
                                        ),
                                ),
                        )
                        // Server list
                        .child(div().flex().flex_col().gap_1().children(
                            mcp_config.mcp_servers.iter().map(|(name, config)| {
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .py_2()
                                    .border_b_1()
                                    .border_color(theme.colors.border)
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(div().size(px(8.0)).rounded_full().bg(
                                                if config.enabled {
                                                    theme.colors.success
                                                } else {
                                                    theme.colors.text_muted
                                                },
                                            ))
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(theme.colors.text)
                                                    .child(name.clone()),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child(config.command.clone()),
                                    )
                            }),
                        ))
                        .when(server_count == 0, |d| {
                            d.child(
                                div()
                                    .py_4()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .gap_2()
                                    .child(div().text_2xl().child("🔌"))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(theme.colors.text_muted)
                                            .child("No MCP servers configured"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.colors.text_muted)
                                            .child("Create an mcp.json file to add servers"),
                                    ),
                            )
                        }),
                    cx,
                ),
            )
            // Config file location
            .child(
                self.render_section(
                    "Configuration",
                    "MCP servers are configured in mcp.json",
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.colors.text_muted)
                                .child("Configuration file locations (in priority order):"),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_family("JetBrains Mono")
                                        .text_color(theme.colors.text_muted)
                                        .child("1. ./mcp.json (current directory)"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_family("JetBrains Mono")
                                        .text_color(theme.colors.text_muted)
                                        .child("2. .mcp/mcp.json (project root)"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_family("JetBrains Mono")
                                        .text_color(theme.colors.text_muted)
                                        .child("3. ~/.config/claude-visual/mcp.json"),
                                ),
                        ),
                    cx,
                ),
            )
    }
}
