use super::super::SettingsModal;
use gpui::prelude::*;
use gpui::*;

impl SettingsModal {
    /// Render a toggle switch
    pub(crate) fn render_toggle(
        &self,
        label: &str,
        value: bool,
        on_change: impl Fn(&mut Self, &mut Context<Self>) + 'static,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);
        let label = label.to_string();

        div()
            .id(SharedString::from(format!("toggle-{}", label)))
            .flex()
            .items_center()
            .justify_between()
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, _window, cx| {
                on_change(this, cx);
            }))
            .child(
                div()
                    .text_sm()
                    .text_color(theme.colors.text)
                    .child(label.clone()),
            )
            .child(
                div()
                    .w(px(40.0))
                    .h(px(22.0))
                    .rounded_full()
                    .p(px(2.0))
                    .when(value, |d| d.bg(theme.colors.accent))
                    .when(!value, |d| d.bg(theme.colors.border))
                    .child(
                        div()
                            .size(px(18.0))
                            .rounded_full()
                            .bg(hsla(0.0, 0.0, 1.0, 1.0))
                            .when(value, |d| d.ml(px(18.0)))
                            .when(!value, |d| d.ml(px(0.0))),
                    ),
            )
    }
}
