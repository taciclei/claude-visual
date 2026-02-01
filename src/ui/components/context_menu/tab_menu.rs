//! Tab context menu component

use gpui::*;
use gpui::prelude::*;
use super::types::*;
use super::menu::ContextMenu;

/// Tab context menu
#[derive(Clone)]
pub struct TabContextMenu {
    tab_name: String,
    is_pinned: bool,
    has_unsaved: bool,
}

impl TabContextMenu {
    pub fn new(tab_name: impl Into<String>) -> Self {
        Self {
            tab_name: tab_name.into(),
            is_pinned: false,
            has_unsaved: false,
        }
    }

    pub fn pinned(mut self, is_pinned: bool) -> Self {
        self.is_pinned = is_pinned;
        self
    }

    pub fn unsaved(mut self, has_unsaved: bool) -> Self {
        self.has_unsaved = has_unsaved;
        self
    }
}

impl RenderOnce for TabContextMenu {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut items = vec![
            ContextMenuItem::action("close", "Close Tab")
                .shortcut("⌘W"),
            ContextMenuItem::action("close_others", "Close Other Tabs"),
            ContextMenuItem::action("close_all", "Close All Tabs"),
        ];

        items.push(ContextMenuItem::separator());

        if self.is_pinned {
            items.push(
                ContextMenuItem::action("unpin", "Unpin Tab")
                    .icon("📍")
            );
        } else {
            items.push(
                ContextMenuItem::action("pin", "Pin Tab")
                    .icon("📌")
            );
        }

        items.push(ContextMenuItem::separator());

        items.push(
            ContextMenuItem::action("duplicate", "Duplicate Tab")
        );

        if self.has_unsaved {
            items.push(ContextMenuItem::separator());
            items.push(
                ContextMenuItem::action("save", "Save")
                    .icon("💾")
                    .shortcut("⌘S")
            );
        }

        ContextMenu::new().items(items)
    }
}
