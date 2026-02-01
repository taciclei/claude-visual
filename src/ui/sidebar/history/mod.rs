//! Conversation history sidebar panel

use gpui::*;

mod core;
mod filters;
mod render;
mod search;
mod types;
mod utils;

pub use core::HistorySidebar;
pub use types::HistorySidebarEvent;

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
