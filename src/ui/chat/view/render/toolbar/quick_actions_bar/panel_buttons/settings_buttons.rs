//! Settings and integration buttons - quick settings and MCP

use crate::ui::chat::view::core::ChatView;
use gpui::prelude::*;
use gpui::*;

impl ChatView {
    /// Render quick settings button (⚙️)
    pub(in crate::ui::chat::view::render::toolbar::quick_actions_bar) fn render_quick_settings_button(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let text_color_active = theme.colors.accent;
        let text_color_inactive = theme.colors.text_muted;
        let text_color_hover = theme.colors.text;
        let surface_hover = theme.colors.surface_hover;

        let on_click = cx.listener(|this, _, _window, cx| {
            this.toggle_quick_settings(cx);
        });

        div()
            .id("quick-settings-btn")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py(px(2.0))
            .rounded_md()
            .cursor_pointer()
            .text_xs()
            .text_color(if self.show_quick_settings {
                text_color_active
            } else {
                text_color_inactive
            })
            .hover(move |s| s.bg(surface_hover).text_color(text_color_hover))
            .on_click(on_click)
            .child("⚙️")
    }

    /// Render MCP servers panel button (🔌 with count)
    /// Left-click: Show quick tools dropdown
    /// Right-click: Show full MCP panel
    pub(in crate::ui::chat::view::render::toolbar::quick_actions_bar) fn render_mcp_panel_button(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let mcp_count = self.mcp_server_count();
        let tool_count = self.mcp_tool_count();
        let text_color_active = theme.colors.accent;
        let text_color_inactive = theme.colors.text_muted;
        let text_color_hover = theme.colors.text;
        let text_color_available = theme.colors.success;
        let surface_hover = theme.colors.surface_hover;

        // Left-click: Quick tools dropdown (fast access)
        let on_click = cx.listener(|this, _, _window, cx| {
            this.toggle_mcp_quick_tools(cx);
        });

        // Right-click: Full MCP panel
        let on_right_click = cx.listener(|this, _: &gpui::MouseDownEvent, _window, cx| {
            this.toggle_mcp_panel(cx);
        });

        div()
            .id("mcp-panel-btn")
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py(px(2.0))
            .rounded_md()
            .cursor_pointer()
            .text_xs()
            .text_color(if self.panels.mcp_panel || self.panels.mcp_quick_tools {
                text_color_active
            } else if tool_count > 0 {
                text_color_available
            } else {
                text_color_inactive
            })
            .hover(move |s| s.bg(surface_hover).text_color(text_color_hover))
            .on_click(on_click)
            .on_mouse_down(gpui::MouseButton::Right, on_right_click)
            .child("🔌")
            .when(tool_count > 0, |d| d.child(format!("{}", tool_count)))
            .when(tool_count == 0 && mcp_count > 0, |d| {
                d.child(format!("{}s", mcp_count))
            })
            .when(tool_count == 0 && mcp_count == 0, |d| d.child("MCP"))
    }
}
