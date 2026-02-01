//! AI menu rendering

use gpui::*;

use crate::ui::debug::debug_panel::{DebugPanel, DebugPanelEvent};
use crate::ui::debug::debug_panel::types::DebugPromptType;

impl DebugPanel {
    /// Render AI prompts menu
    pub(in crate::ui::debug::debug_panel) fn render_ai_menu(&self, theme: &crate::app::theme::Theme, cx: &Context<Self>) -> impl IntoElement {
        // Copy theme colors for move closures
        let surface_color = theme.colors.surface;
        let border_color = theme.colors.border;
        let text_muted_color = theme.colors.text_muted;
        let surface_hover_color = theme.colors.surface_hover;
        let accent_color = theme.colors.accent;
        let text_color = theme.colors.text;

        div()
            .absolute()
            .top(px(40.0))
            .right(px(8.0))
            .w(px(200.0))
            .bg(surface_color)
            .border_1()
            .border_color(border_color)
            .rounded_md()
            .shadow_lg()
            .p_2()
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_muted_color)
                    .mb_1()
                    .child("AI Debug Assistance"),
            )
            .children(DebugPromptType::all().into_iter().map(|prompt_type| {
                let on_click = cx.listener(move |this, _, _window, cx| {
                    this.close_ai_menu(cx);
                    cx.emit(DebugPanelEvent::AskAI(prompt_type));
                });

                div()
                    .id(ElementId::Name(format!("ai-prompt-{:?}", prompt_type).into()))
                    .px_2()
                    .py_1()
                    .rounded_sm()
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .gap_2()
                    .hover(|s| s.bg(surface_hover_color))
                    .on_click(on_click)
                    .child(
                        div()
                            .w(px(20.0))
                            .text_xs()
                            .text_color(accent_color)
                            .child(prompt_type.icon()),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(text_color)
                            .child(prompt_type.label()),
                    )
            }))
    }
}
