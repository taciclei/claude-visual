//! Call Stack Types
//!
//! Data structures for call stack representation.

/// Stack frame item for display
#[derive(Debug, Clone)]
pub struct StackFrameItem {
    /// Frame ID
    pub id: i64,
    /// Function/method name
    pub name: String,
    /// Source file path
    pub source_path: Option<String>,
    /// Source file name
    pub source_name: Option<String>,
    /// Line number
    pub line: i64,
    /// Column
    pub column: i64,
    /// Module name
    pub module: Option<String>,
    /// Is current frame
    pub is_current: bool,
}

impl StackFrameItem {
    /// Get display name
    pub fn display_name(&self) -> &str {
        &self.name
    }

    /// Get location string
    pub fn location(&self) -> String {
        if let Some(name) = &self.source_name {
            format!("{}:{}", name, self.line)
        } else {
            format!("line {}", self.line)
        }
    }

    /// Get short location
    pub fn short_location(&self) -> String {
        if let Some(name) = &self.source_name {
            format!("{}:{}", name, self.line)
        } else {
            format!(":{}", self.line)
        }
    }
}

/// Thread item for display
#[derive(Debug, Clone)]
pub struct ThreadItem {
    /// Thread ID
    pub id: i64,
    /// Thread name
    pub name: String,
    /// Stack frames for this thread
    pub frames: Vec<StackFrameItem>,
    /// Is expanded
    pub expanded: bool,
    /// Is current thread
    pub is_current: bool,
}

impl ThreadItem {
    /// Create a new thread item
    pub fn new(id: i64, name: String) -> Self {
        Self {
            id,
            name,
            frames: Vec::new(),
            expanded: true,
            is_current: false,
        }
    }

    /// Get frame count
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }
}

/// Events from call stack view
#[derive(Debug, Clone)]
pub enum CallStackViewEvent {
    /// Select a frame
    SelectFrame { thread_id: i64, frame_id: i64 },
    /// Toggle thread expanded
    ToggleThread(i64),
    /// Refresh stack
    Refresh,
    /// Copy stack trace
    CopyStackTrace,
}
