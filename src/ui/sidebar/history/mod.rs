//! Conversation history sidebar panel

use gpui::*;

mod types;
mod core;
mod search;
mod filters;
mod utils;
mod render;

pub use types::HistorySidebarEvent;
pub use core::HistorySidebar;

impl EventEmitter<HistorySidebarEvent> for HistorySidebar {}

impl Focusable for HistorySidebar {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests can be added here
}
