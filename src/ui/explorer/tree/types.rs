//! Type definitions for file tree component

use gpui::*;
use std::path::PathBuf;

/// Simple color palette for theming
pub(crate) struct SimpleColors {
    pub(crate) surface: Hsla,
    pub(crate) surface_hover: Hsla,
    pub(crate) border: Hsla,
    pub(crate) text: Hsla,
    pub(crate) text_muted: Hsla,
    pub(crate) accent: Hsla,
    pub(crate) error: Hsla,
    pub(crate) success: Hsla,
    pub(crate) warning: Hsla,
    pub(crate) background: Hsla,
}

pub(crate) fn default_colors() -> SimpleColors {
    SimpleColors {
        surface: hsla(220.0 / 360.0, 0.13, 0.12, 1.0),
        surface_hover: hsla(220.0 / 360.0, 0.13, 0.15, 1.0),
        border: hsla(220.0 / 360.0, 0.13, 0.20, 1.0),
        text: hsla(0.0, 0.0, 0.93, 1.0),
        text_muted: hsla(0.0, 0.0, 0.60, 1.0),
        accent: hsla(210.0 / 360.0, 0.80, 0.55, 1.0),
        error: hsla(0.0, 0.84, 0.60, 1.0),
        success: hsla(142.0 / 360.0, 0.71, 0.45, 1.0),
        warning: hsla(38.0 / 360.0, 0.92, 0.50, 1.0),
        background: hsla(220.0 / 360.0, 0.13, 0.09, 1.0),
    }
}

/// Drag preview for files
pub(crate) struct DragPreview {
    pub(crate) is_directory: bool,
    pub(crate) name: String,
}

impl Render for DragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_1()
            .px_3()
            .py_1()
            .rounded_md()
            .bg(hsla(210.0 / 360.0, 0.3, 0.3, 0.95))
            .border_1()
            .border_color(hsla(210.0 / 360.0, 0.5, 0.5, 0.5))
            .shadow_lg()
            .text_sm()
            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
            .child(if self.is_directory { "📁" } else { "📄" })
            .child(self.name.clone())
    }
}

/// File tree events
#[derive(Clone, Debug)]
pub enum FileTreeEvent {
    /// File selected (single click)
    FileSelected(PathBuf),
    /// File opened (double click)
    FileOpened(PathBuf),
    /// File added to context
    FileAddedToContext(PathBuf),
    /// File renamed
    FileRenamed { from: PathBuf, to: PathBuf },
    /// File deleted
    FileDeleted(PathBuf),
    /// New file requested
    NewFileRequested(PathBuf), // Parent directory
    /// New folder requested
    NewFolderRequested(PathBuf), // Parent directory
    /// Refresh requested
    RefreshRequested,
    /// File drag started
    FileDragStarted(PathBuf),
    /// Files drag started (multiple)
    FilesDragStarted(Vec<PathBuf>),
}

/// Dragged file data for context attachment
#[derive(Clone, Debug)]
pub struct DraggedFile {
    /// File path
    pub path: PathBuf,
    /// File name for display
    pub name: String,
    /// Whether it's a directory
    pub is_directory: bool,
}

impl DraggedFile {
    pub fn new(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.display().to_string());
        let is_directory = path.is_dir();
        Self {
            path,
            name,
            is_directory,
        }
    }
}

/// Multiple dragged files
#[derive(Clone, Debug)]
pub struct DraggedFiles {
    pub files: Vec<DraggedFile>,
}

impl DraggedFiles {
    pub fn new(paths: Vec<PathBuf>) -> Self {
        Self {
            files: paths.into_iter().map(DraggedFile::new).collect(),
        }
    }

    pub fn count(&self) -> usize {
        self.files.len()
    }

    pub fn paths(&self) -> Vec<PathBuf> {
        self.files.iter().map(|f| f.path.clone()).collect()
    }
}
