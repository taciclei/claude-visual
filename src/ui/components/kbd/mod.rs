//! Keyboard shortcut display components

mod common_shortcuts;
mod kbd;
mod keyboard_shortcut;
mod shortcut_hint;
mod shortcut_list;
mod types;

pub use common_shortcuts::CommonShortcuts;
pub use kbd::Kbd;
pub use keyboard_shortcut::KeyboardShortcut;
pub use shortcut_hint::ShortcutHint;
pub use shortcut_list::ShortcutList;
pub use types::{KbdSize, KbdStyle, Platform};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kbd_creation() {
        let key = Kbd::new("A").size(KbdSize::Large).style(KbdStyle::Flat);

        assert_eq!(key.key, "A");
        assert_eq!(key.size, KbdSize::Large);
        assert_eq!(key.style, KbdStyle::Flat);
    }

    #[test]
    fn test_keyboard_shortcut_parse() {
        let shortcut = KeyboardShortcut::parse("Cmd+Shift+P");
        assert_eq!(shortcut.keys.len(), 3);
        assert_eq!(shortcut.keys[0], "⌘");
        assert_eq!(shortcut.keys[1], "⇧");
        assert_eq!(shortcut.keys[2], "P");
    }

    #[test]
    fn test_keyboard_shortcut_parse_modifiers() {
        let shortcut = KeyboardShortcut::parse("Ctrl+Alt+Delete");
        assert_eq!(shortcut.keys[0], "⌃");
        assert_eq!(shortcut.keys[1], "⌥");
        assert_eq!(shortcut.keys[2], "⌫");
    }

    #[test]
    fn test_shortcut_list() {
        let list = ShortcutList::new()
            .title("Test")
            .shortcut("Save", "Cmd+S")
            .shortcut("Copy", "Cmd+C");

        assert_eq!(list.shortcuts.len(), 2);
    }
}
