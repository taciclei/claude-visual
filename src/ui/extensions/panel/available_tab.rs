use gpui::*;
use gpui::prelude::*;

use super::panel::ExtensionsPanel;

impl ExtensionsPanel {
    /// Render the available tab (marketplace placeholder)
    pub(super) fn render_available_tab(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        div()
            .flex_1()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                div()
                    .text_3xl()
                    .child("Coming Soon"),
            )
            .child(
                div()
                    .text_base()
                    .text_color(theme.colors.text_muted)
                    .child("Extension marketplace will be available in a future update"),
            )
    }
}
