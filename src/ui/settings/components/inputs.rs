use super::super::SettingsModal;
use crate::ui::pct;
use gpui::prelude::*;
use gpui::*;

impl SettingsModal {
    /// Render a slider
    pub(crate) fn render_slider(
        &self,
        label: &str,
        value: f32,
        min: f32,
        max: f32,
        _on_change: impl Fn(&mut Self, f32, &mut Context<Self>) + 'static,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let percentage = ((value - min) / (max - min) * 100.0) as i32;
        let label = label.to_string();

        div()
            .flex()
            .items_center()
            .justify_between()
            .child(div().text_sm().text_color(theme.colors.text).child(label))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Slider track (visual only for now)
                    .child(
                        div()
                            .w(px(100.0))
                            .h(px(4.0))
                            .rounded_full()
                            .bg(theme.colors.border)
                            .child(
                                div()
                                    .h_full()
                                    .rounded_full()
                                    .bg(theme.colors.accent)
                                    .w(pct(percentage as f32)),
                            ),
                    )
                    // Value display
                    .child(
                        div()
                            .w(px(40.0))
                            .text_xs()
                            .text_color(theme.colors.text_muted)
                            .child(format!("{:.0}", value)),
                    ),
            )
    }
}
