//! Tab navigation rendering

use gpui::*;
use gpui::prelude::*;

use crate::ui::debug::debug_panel::DebugPanel;
use crate::ui::debug::debug_panel::types::DebugTab;

impl DebugPanel {
    /// Render tabs
    pub(in crate::ui::debug::debug_panel) fn render_tabs(&self, theme: &crate::app::theme::Theme, cx: &Context<Self>) -> impl IntoElement {
        let tabs = [
            DebugTab::Console,
            DebugTab::Variables,
            DebugTab::CallStack,
            DebugTab::Breakpoints,
            DebugTab::Watch,
        ];

        // Copy theme colors for move closures
        let text_color = theme.colors.text;
        let border_color = theme.colors.border;
        let accent_color = theme.colors.accent;
        let text_muted_color = theme.colors.text_muted;

        div()
            .flex()
            .items_center()
            .border_b_1()
            .border_color(border_color)
            .children(tabs.into_iter().map(|tab| {
                let is_active = self.active_tab == tab;
                let on_click = cx.listener(move |this, _, _window, cx| {
                    this.set_tab(tab, cx);
                });

                div()
                    .id(SharedString::from(format!("debug-tab-{:?}", tab)))
                    .px_3()
                    .py_1()
                    .text_xs()
                    .cursor_pointer()
                    .when(is_active, |d| {
                        d.text_color(text_color)
                            .border_b_2()
                            .border_color(accent_color)
                    })
                    .when(!is_active, |d| {
                        d.text_color(text_muted_color)
                            .hover(|s| s.text_color(text_color))
                    })
                    .on_click(on_click)
                    .child(tab.label())
            }))
    }
}
