//! Context menu render functions for ChatView

use super::super::core::ChatView;
use crate::claude::message::MessageRole;
use gpui::prelude::*;
use gpui::*;

impl ChatView {
    pub fn render_context_menu(
        &self,
        theme: &crate::app::theme::Theme,
        cx: &mut Context<Self>,
    ) -> Div {
        let menu = match &self.context_menu {
            Some(m) => m.clone(),
            None => return div(),
        };

        let message_index = menu.message_index;
        let is_user_message = self
            .messages
            .get(message_index)
            .map(|m| matches!(m.role, MessageRole::User))
            .unwrap_or(false);
        let is_pinned = self.pinned_messages.contains(&message_index);
        let is_bookmarked = self.bookmarked_messages.contains(&message_index);

        // Check if message contains code
        let has_code = self
            .messages
            .get(message_index)
            .map(|m| m.content.contains("```"))
            .unwrap_or(false);

        // Menu items: (icon, label, action, enabled)
        let mut menu_items: Vec<(&str, &str, &str, bool)> = vec![
            ("📋", "Copy", "copy", true),
            ("📝", "Copy as Markdown", "copy_as_markdown", true),
            ("💬", "Quote", "quote", true),
            ("📌", if is_pinned { "Unpin" } else { "Pin" }, "pin", true),
            (
                "🔖",
                if is_bookmarked {
                    "Unbookmark"
                } else {
                    "Bookmark"
                },
                "bookmark",
                true,
            ),
            ("😊", "React", "react", true),
            ("✏️", "Edit", "edit", is_user_message),
            ("🔀", "Branch from Here", "branch", is_user_message),
            ("🔄", "Retry from Here", "retry_from_here", is_user_message),
        ];

        // Add Claude Code skill actions for assistant messages with code
        if !is_user_message && has_code {
            menu_items.push(("---", "─────────", "separator", false));
            menu_items.push(("📖", "Explain Code", "skill_explain", true));
            menu_items.push(("👀", "Review Code", "skill_review", true));
            menu_items.push(("♻️", "Refactor", "skill_refactor", true));
            menu_items.push(("🧪", "Generate Tests", "skill_tests", true));
        }

        // Add general AI actions
        if !is_user_message {
            menu_items.push(("---", "─────────", "separator", false));
            menu_items.push(("💡", "Continue", "continue", true));
            menu_items.push(("🔍", "Explain More", "explain_more", true));
        }

        menu_items.push(("---", "─────────", "separator", false));
        menu_items.push(("🗑️", "Delete", "delete", true));

        div().absolute().inset_0().child(
            div()
                .absolute()
                .top(px(menu.y))
                .left(px(menu.x))
                .w(px(160.0))
                .bg(theme.colors.surface)
                .rounded_lg()
                .border_1()
                .border_color(theme.colors.border)
                .shadow_lg()
                .overflow_hidden()
                .child(
                    div().flex().flex_col().py_1().children(
                        menu_items
                            .into_iter()
                            .filter(|(_, _, _, enabled)| *enabled)
                            .map(|(icon, label, action, _)| {
                                let action_str = action.to_string();
                                div()
                                    .id(ElementId::Name(format!("ctx-menu-{}", action).into()))
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .px_3()
                                    .py_2()
                                    .cursor_pointer()
                                    .text_sm()
                                    .text_color(theme.colors.text)
                                    .hover(|s| s.bg(theme.colors.surface_hover))
                                    .on_click(cx.listener(move |this, _, _window, cx| {
                                        this.execute_context_menu_action(&action_str, cx);
                                    }))
                                    .child(div().text_sm().child(icon))
                                    .child(label)
                            }),
                    ),
                ),
        )
    }
}
