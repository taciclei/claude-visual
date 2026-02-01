//! Edit context menu component

use gpui::*;
use gpui::prelude::*;
use super::types::*;
use super::menu::ContextMenu;

/// Edit context menu (for text fields)
#[derive(Clone)]
pub struct EditContextMenu {
    has_selection: bool,
    can_paste: bool,
    can_undo: bool,
    can_redo: bool,
}

impl EditContextMenu {
    pub fn new() -> Self {
        Self {
            has_selection: false,
            can_paste: true,
            can_undo: false,
            can_redo: false,
        }
    }

    pub fn has_selection(mut self, has: bool) -> Self {
        self.has_selection = has;
        self
    }

    pub fn can_paste(mut self, can: bool) -> Self {
        self.can_paste = can;
        self
    }

    pub fn can_undo(mut self, can: bool) -> Self {
        self.can_undo = can;
        self
    }

    pub fn can_redo(mut self, can: bool) -> Self {
        self.can_redo = can;
        self
    }
}

impl Default for EditContextMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for EditContextMenu {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut items = Vec::new();

        // Undo/Redo
        let mut undo = ContextMenuItem::action("undo", "Undo")
            .icon("↶")
            .shortcut("⌘Z");
        if !self.can_undo {
            undo = undo.disabled();
        }
        items.push(undo);

        let mut redo = ContextMenuItem::action("redo", "Redo")
            .icon("↷")
            .shortcut("⇧⌘Z");
        if !self.can_redo {
            redo = redo.disabled();
        }
        items.push(redo);

        items.push(ContextMenuItem::separator());

        // Cut/Copy/Paste
        let mut cut = ContextMenuItem::action("cut", "Cut")
            .icon("✂️")
            .shortcut("⌘X");
        if !self.has_selection {
            cut = cut.disabled();
        }
        items.push(cut);

        let mut copy = ContextMenuItem::action("copy", "Copy")
            .icon("📄")
            .shortcut("⌘C");
        if !self.has_selection {
            copy = copy.disabled();
        }
        items.push(copy);

        let mut paste = ContextMenuItem::action("paste", "Paste")
            .icon("📋")
            .shortcut("⌘V");
        if !self.can_paste {
            paste = paste.disabled();
        }
        items.push(paste);

        items.push(ContextMenuItem::separator());

        items.push(
            ContextMenuItem::action("select_all", "Select All")
                .shortcut("⌘A")
        );

        ContextMenu::new().items(items).min_width(160.0)
    }
}
