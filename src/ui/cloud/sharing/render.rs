//! Share dialog rendering entry point

use gpui::*;
use gpui::prelude::*;

use super::dialog::ShareDialog;

impl Render for ShareDialog {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx);

        // Modal backdrop
        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(Hsla::from(rgb(0x000000)).opacity(0.5))
            .child(
                // Modal content
                div()
                    .w(px(480.0))
                    .max_h(px(700.0))
                    .bg(theme.colors.background)
                    .rounded_xl()
                    .border_1()
                    .border_color(theme.colors.border)
                    .shadow_xl()
                    .p_6()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .id("scroll-share-dialog")
                    .overflow_y_scroll()
                    .child(self.render_header(cx))
                    .child(self.render_error(cx))
                    .child(self.render_permission_selector(cx))
                    .child(self.render_options(cx))
                    .child(self.render_generate_button(cx))
                    .child(self.render_existing_links(cx)),
            )
    }
}
