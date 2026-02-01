//! Rendering implementation for FilePreviewPanel

mod components;
mod states;

use gpui::*;

use super::core::FilePreviewPanel;
use super::types::PreviewState;

impl Render for FilePreviewPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match &self.state {
            PreviewState::Empty => self.render_empty(cx),
            PreviewState::Loading(path) => self.render_loading(path, cx),
            PreviewState::Loaded {
                path,
                content,
                line_count,
                file_size,
                language,
            } => self.render_loaded(path, content, *line_count, *file_size, language.as_deref(), cx),
            PreviewState::Binary { path, file_size } => self.render_binary(path, *file_size, cx),
            PreviewState::TooLarge { path, file_size } => self.render_too_large(path, *file_size, cx),
            PreviewState::Error { path, message } => self.render_error(path, message, cx),
        }
    }
}
