//! Common keyboard shortcuts reference

use gpui::*;
use gpui::prelude::*;
use super::shortcut_list::ShortcutList;

/// Common keyboard shortcuts reference
#[derive(Clone)]
pub struct CommonShortcuts;

impl CommonShortcuts {
    /// File operations shortcuts
    pub fn file_operations() -> ShortcutList {
        ShortcutList::new()
            .title("File Operations")
            .shortcut("New File", "Cmd+N")
            .shortcut("Open File", "Cmd+O")
            .shortcut("Save", "Cmd+S")
            .shortcut("Save As", "Cmd+Shift+S")
            .shortcut("Close", "Cmd+W")
    }

    /// Edit shortcuts
    pub fn editing() -> ShortcutList {
        ShortcutList::new()
            .title("Editing")
            .shortcut("Undo", "Cmd+Z")
            .shortcut("Redo", "Cmd+Shift+Z")
            .shortcut("Cut", "Cmd+X")
            .shortcut("Copy", "Cmd+C")
            .shortcut("Paste", "Cmd+V")
            .shortcut("Select All", "Cmd+A")
            .shortcut("Find", "Cmd+F")
            .shortcut("Replace", "Cmd+H")
    }

    /// Navigation shortcuts
    pub fn navigation() -> ShortcutList {
        ShortcutList::new()
            .title("Navigation")
            .shortcut("Go to File", "Cmd+P")
            .shortcut("Go to Line", "Ctrl+G")
            .shortcut("Go to Symbol", "Cmd+Shift+O")
            .shortcut("Back", "Ctrl+-")
            .shortcut("Forward", "Ctrl+Shift+-")
    }
}

impl RenderOnce for CommonShortcuts {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(Self::file_operations())
            .child(Self::editing())
            .child(Self::navigation())
    }
}
