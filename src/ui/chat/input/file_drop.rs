//! File drop handlers for chat input

use gpui::*;

use crate::ui::explorer::{DraggedFile, DraggedFiles};
use super::{ChatInput, ChatInputEvent};

impl ChatInput {
    /// Handle file dropped from file explorer
    pub fn handle_file_drop(&mut self, file: &DraggedFile, _window: &mut Window, cx: &mut Context<Self>) {
        self.is_drag_over = false;
        // Add file mention
        let mention_text = format!("@file:{} ", file.path.display());
        self.text.push_str(&mention_text);
        self.cursor_position = self.text.len();
        self.update_mentions();
        // Emit files attached event
        cx.emit(ChatInputEvent::FilesAttached(vec![file.path.clone()]));
        cx.notify();
    }

    /// Handle multiple files dropped
    pub fn handle_files_drop(&mut self, files: &DraggedFiles, _window: &mut Window, cx: &mut Context<Self>) {
        self.is_drag_over = false;
        // Add file mentions
        for file in &files.files {
            let mention_text = format!("@file:{} ", file.path.display());
            self.text.push_str(&mention_text);
        }
        self.cursor_position = self.text.len();
        self.update_mentions();
        // Emit files attached event
        cx.emit(ChatInputEvent::FilesAttached(files.paths()));
        cx.notify();
    }
}
