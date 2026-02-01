//! Trait implementations

use gpui::*;
use super::core::TerminalView;
use super::types::TerminalViewEvent;

impl EventEmitter<TerminalViewEvent> for TerminalView {}

impl Focusable for TerminalView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
