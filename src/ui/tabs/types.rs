use gpui::*;

/// Events emitted by the TabBar
pub enum TabBarEvent {
    /// A tab was selected
    TabSelected(usize),
    /// A new tab was requested
    NewTabRequested,
    /// A tab close was requested
    CloseTabRequested(usize),
    /// Tabs were reordered via drag
    TabsReordered { from: usize, to: usize },
}

/// Represents a single conversation tab
#[derive(Debug, Clone)]
pub struct Tab {
    /// Unique identifier for this tab
    pub id: String,
    /// Display title (e.g., "New Chat" or first message preview)
    pub title: String,
    /// Optional conversation ID if persisted
    pub conversation_id: Option<String>,
    /// Whether this tab has unsaved changes
    pub is_dirty: bool,
    /// Whether this tab is pinned
    pub is_pinned: bool,
}

impl Tab {
    /// Create a new empty tab
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: "New Chat".to_string(),
            conversation_id: None,
            is_dirty: false,
            is_pinned: false,
        }
    }

    /// Create a tab from an existing conversation
    pub fn from_conversation(conversation_id: String, title: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            conversation_id: Some(conversation_id),
            is_dirty: false,
            is_pinned: false,
        }
    }

    /// Pin this tab
    pub fn pin(&mut self) {
        self.is_pinned = true;
    }

    /// Unpin this tab
    pub fn unpin(&mut self) {
        self.is_pinned = false;
    }
}

impl Default for Tab {
    fn default() -> Self {
        Self::new()
    }
}

/// Dragged tab data for reordering
#[derive(Clone)]
pub struct DraggedTab {
    pub index: usize,
    pub tab: Tab,
}

/// Drag preview for tabs
pub(super) struct TabDragPreview {
    pub(super) title: String,
}

impl Render for TabDragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .px_3()
            .py_1()
            .rounded_md()
            .bg(hsla(0.0, 0.0, 0.5, 0.8))
            .text_sm()
            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
            .child(self.title.clone())
    }
}

/// Events emitted for tab pinning
pub enum TabPinEvent {
    /// Tab was pinned
    Pinned(usize),
    /// Tab was unpinned
    Unpinned(usize),
}
