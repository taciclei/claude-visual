use super::core::ModelSelector;
use super::types::ModelSelectorEvent;
use gpui::*;

impl EventEmitter<ModelSelectorEvent> for ModelSelector {}

impl Focusable for ModelSelector {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
