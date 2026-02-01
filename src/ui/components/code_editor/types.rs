//! Shared types for code editor components

use gpui::*;

/// Editor theme
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum EditorTheme {
    #[default]
    Dark,
    Light,
    Monokai,
    Dracula,
    Nord,
}

impl EditorTheme {
    pub(crate) fn background(&self) -> gpui::Hsla {
        match self {
            Self::Dark => hsla(0.0, 0.0, 0.08, 1.0),
            Self::Light => hsla(0.0, 0.0, 0.98, 1.0),
            Self::Monokai => hsla(0.17, 0.17, 0.16, 1.0),
            Self::Dracula => hsla(0.72, 0.21, 0.18, 1.0),
            Self::Nord => hsla(0.58, 0.16, 0.18, 1.0),
        }
    }

    pub(crate) fn text_color(&self) -> gpui::Hsla {
        match self {
            Self::Dark => hsla(0.0, 0.0, 0.9, 1.0),
            Self::Light => hsla(0.0, 0.0, 0.15, 1.0),
            Self::Monokai => hsla(0.16, 0.03, 0.98, 1.0),
            Self::Dracula => hsla(0.0, 0.0, 0.97, 1.0),
            Self::Nord => hsla(0.54, 0.16, 0.88, 1.0),
        }
    }

    pub(crate) fn line_number_color(&self) -> gpui::Hsla {
        match self {
            Self::Dark => hsla(0.0, 0.0, 0.4, 1.0),
            Self::Light => hsla(0.0, 0.0, 0.6, 1.0),
            Self::Monokai => hsla(0.0, 0.0, 0.5, 1.0),
            Self::Dracula => hsla(0.0, 0.0, 0.45, 1.0),
            Self::Nord => hsla(0.0, 0.0, 0.45, 1.0),
        }
    }

    pub(crate) fn selection_color(&self) -> gpui::Hsla {
        match self {
            Self::Dark => hsla(0.6, 0.5, 0.4, 0.3),
            Self::Light => hsla(0.6, 0.7, 0.7, 0.3),
            Self::Monokai => hsla(0.15, 0.5, 0.4, 0.3),
            Self::Dracula => hsla(0.72, 0.3, 0.4, 0.3),
            Self::Nord => hsla(0.58, 0.3, 0.4, 0.3),
        }
    }
}

/// Editor font size
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum EditorFontSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl EditorFontSize {
    pub(crate) fn size(&self) -> f32 {
        match self {
            Self::Xs => 11.0,
            Self::Sm => 12.0,
            Self::Md => 13.0,
            Self::Lg => 14.0,
            Self::Xl => 16.0,
        }
    }

    pub(crate) fn line_height(&self) -> f32 {
        self.size() * 1.5
    }
}

/// Code editor line
#[derive(Clone)]
pub struct EditorLine {
    pub number: usize,
    pub content: SharedString,
    pub is_modified: bool,
    pub has_error: bool,
    pub has_warning: bool,
    pub is_breakpoint: bool,
    pub is_current: bool,
}

impl EditorLine {
    pub fn new(number: usize, content: impl Into<SharedString>) -> Self {
        Self {
            number,
            content: content.into(),
            is_modified: false,
            has_error: false,
            has_warning: false,
            is_breakpoint: false,
            is_current: false,
        }
    }

    pub fn modified(mut self) -> Self {
        self.is_modified = true;
        self
    }

    pub fn error(mut self) -> Self {
        self.has_error = true;
        self
    }

    pub fn warning(mut self) -> Self {
        self.has_warning = true;
        self
    }

    pub fn breakpoint(mut self) -> Self {
        self.is_breakpoint = true;
        self
    }

    pub fn current(mut self) -> Self {
        self.is_current = true;
        self
    }
}

/// Selection range
#[derive(Clone, Debug)]
pub struct Selection {
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

impl Selection {
    pub fn new(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Self {
        Self {
            start_line,
            start_col,
            end_line,
            end_col,
        }
    }

    pub fn cursor(line: usize, col: usize) -> Self {
        Self {
            start_line: line,
            start_col: col,
            end_line: line,
            end_col: col,
        }
    }
}
