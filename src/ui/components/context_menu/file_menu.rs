//! File context menu component

use gpui::*;
use gpui::prelude::*;
use super::types::*;
use super::menu::ContextMenu;

/// Simple file context menu
#[derive(Clone)]
pub struct FileContextMenu {
    filename: String,
    is_directory: bool,
    can_rename: bool,
    can_delete: bool,
}

impl FileContextMenu {
    pub fn new(filename: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
            is_directory: false,
            can_rename: true,
            can_delete: true,
        }
    }

    pub fn directory(mut self) -> Self {
        self.is_directory = true;
        self
    }

    pub fn read_only(mut self) -> Self {
        self.can_rename = false;
        self.can_delete = false;
        self
    }
}

impl RenderOnce for FileContextMenu {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut items = vec![
            ContextMenuItem::action("open", if self.is_directory { "Open Folder" } else { "Open" })
                .icon("📂")
                .shortcut("Enter"),
        ];

        if !self.is_directory {
            items.push(
                ContextMenuItem::action("open_with", "Open With...")
                    .icon("📋")
            );
        }

        items.push(ContextMenuItem::separator());

        items.push(
            ContextMenuItem::action("copy", "Copy")
                .icon("📄")
                .shortcut("⌘C")
        );

        items.push(
            ContextMenuItem::action("cut", "Cut")
                .icon("✂️")
                .shortcut("⌘X")
        );

        items.push(ContextMenuItem::separator());

        if self.can_rename {
            items.push(
                ContextMenuItem::action("rename", "Rename")
                    .icon("✏️")
                    .shortcut("Enter")
            );
        }

        if self.can_delete {
            items.push(
                ContextMenuItem::action("delete", "Delete")
                    .icon("🗑️")
                    .shortcut("⌘⌫")
                    .danger()
            );
        }

        items.push(ContextMenuItem::separator());

        items.push(
            ContextMenuItem::action("info", "Get Info")
                .icon("ℹ️")
                .shortcut("⌘I")
        );

        ContextMenu::new().items(items)
    }
}
