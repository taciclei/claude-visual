//! Trait implementations

use super::core::TerminalView;
use super::types::TerminalViewEvent;
use gpui::*;

impl EventEmitter<TerminalViewEvent> for TerminalView {}

impl Focusable for TerminalView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
