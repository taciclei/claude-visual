//! Messages toolbar render functions

mod title_editor;
mod collapse_expand;
mod filter_chips;
mod navigation;
mod actions;
pub mod view_options;

pub use title_editor::*;
pub use collapse_expand::*;
pub use filter_chips::*;
pub use navigation::*;
pub use actions::*;

use gpui::*;
use gpui::prelude::*;
use crate::ui::chat::view::core::ChatView;

impl ChatView {
    /// Renders the messages toolbar with all controls
    pub fn render_messages_toolbar(&self, theme: &crate::app::theme::Theme, cx: &mut Context<Self>) -> Div {
        let has_messages = !self.message_views.is_empty();

        div()
            .flex()
            .items_center()
            .justify_between()
            .px_4()
            .py_1()
            .bg(theme.colors.surface)
            .border_b_1()
            .border_color(theme.colors.border)
            // Left side - title and view controls
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(self.render_title_section(theme, cx))
                    // Separator after title
                    .child(
                        div()
                            .w(px(1.0))
                            .h(px(16.0))
                            .bg(theme.colors.border)
                            .mx_1()
                    )
                    // Collapse/expand buttons (when has messages)
                    .when(has_messages, |d| {
                        d.child(self.render_collapse_expand_buttons(theme, cx))
                    })
                    // Separator
                    .when(has_messages, |d| {
                        d.child(
                            div()
                                .w(px(1.0))
                                .h(px(16.0))
                                .bg(theme.colors.border)
                                .mx_2()
                        )
                    })
                    // Filter chips (when has messages)
                    .when(has_messages, |d| {
                        d.child(self.render_filter_chips(theme, cx))
                    })
                    // Navigation indicator (when message selected or navigation hint)
                    .when(has_messages, |d| {
                        d.child(self.render_navigation_indicator(theme, cx))
                    })
            )
            // Right side - actions and view options
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Conversation actions (when has messages)
                    .when(has_messages, |d| {
                        d.child(self.render_conversation_actions(theme, cx))
                    })
                    // Separator before view options
                    .child(
                        div()
                            .w(px(1.0))
                            .h(px(16.0))
                            .bg(theme.colors.border)
                    )
                    // View options
                    .child(self.render_view_options(theme, cx))
            )
    }
}
