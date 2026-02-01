//! Content panel rendering

use gpui::*;

use crate::ui::debug::debug_panel::DebugPanel;

impl DebugPanel {
    /// Render console output
    pub(in crate::ui::debug::debug_panel) fn render_console(&self, theme: &crate::app::theme::Theme) -> impl IntoElement {
        // Copy theme colors for move closures
        let background_color = theme.colors.background;
        let text_color = theme.colors.text;
        let error_color = theme.colors.error;
        let text_muted_color = theme.colors.text_muted;
        let accent_color = theme.colors.accent;

        div()
            .flex_1()
            .id("scroll-debug-console")
            .overflow_y_scroll()
            .p_2()
            .bg(background_color)
            .font_family("JetBrains Mono")
            .text_xs()
            .children(self.output.iter().map(|line| {
                let color = match line.category.as_str() {
                    "stdout" => text_color,
                    "stderr" => error_color,
                    "console" => text_muted_color,
                    "debug" => accent_color,
                    _ => text_muted_color,
                };

                div()
                    .w_full()
                    .py_0p5()
                    .text_color(color)
                    .whitespace_nowrap()
                    .child(line.text.clone())
            }))
    }
}
