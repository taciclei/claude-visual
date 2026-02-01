//! Settings modal rendering

mod header;
mod sidebar;
mod content;
mod footer;

use gpui::*;
use gpui::prelude::*;
use crate::ui::pct;
use super::core::SettingsModal;

impl Render for SettingsModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.app_state.theme.read(cx).clone();

        let on_backdrop_click = cx.listener(|this, _, _window, cx| {
            this.dismiss(cx);
        });
        let on_key_down = cx.listener(|this, event: &KeyDownEvent, _window, cx| {
            if event.keystroke.key == "escape" {
                this.dismiss(cx);
            }
        });

        let bg_color = theme.colors.background;
        let surface_color = theme.colors.surface;
        let border_color = theme.colors.border;

        // Backdrop
        div()
            .id("settings-modal-backdrop")
            .track_focus(&self.focus_handle)
            .absolute()
            .inset_0()
            .bg(bg_color.opacity(0.8))
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(MouseButton::Left, on_backdrop_click)
            // Escape to close
            .on_key_down(on_key_down)
            // Modal container
            .child(
                div()
                    .id("settings-modal")
                    .w(px(700.0))
                    .max_h(pct(80.0))
                    .rounded_xl()
                    .bg(surface_color)
                    .border_1()
                    .border_color(border_color)
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    // Prevent clicks from closing modal
                    .on_mouse_down(MouseButton::Left, |_, _window, _cx| {
                    })
                    // Header
                    .child(self.render_header(&theme, cx))
                    // Content area
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_row()
                            .overflow_hidden()
                            // Sidebar with tabs
                            .child(self.render_sidebar(&theme, cx))
                            // Tab content
                            .child(self.render_content(cx)),
                    )
                    // Footer with save button
                    .child(self.render_footer(&theme, cx)),
            )
            // Reset confirmation dialog
            .when(self.show_reset_confirmation, |this| {
                this.child(
                    self.render_reset_confirmation_dialog(&theme, cx)
                )
            })
            // Import/Export dialog
            .when(self.show_import_export, |this| {
                this.child(
                    self.render_import_export_dialog(&theme, cx)
                )
            })
    }
}
