//! Shared types for tree components

/// Tree node data
#[derive(Debug, Clone)]
pub struct TreeNode {
    /// Unique identifier
    pub id: String,
    /// Display label
    pub label: String,
    /// Optional icon
    pub icon: Option<String>,
    /// Child nodes
    pub children: Vec<TreeNode>,
    /// Whether node is selectable
    pub selectable: bool,
    /// Whether node is disabled
    pub disabled: bool,
    /// Optional metadata
    pub data: Option<String>,
}

impl TreeNode {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            children: Vec::new(),
            selectable: true,
            disabled: false,
            data: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn child(mut self, child: TreeNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }

    pub fn not_selectable(mut self) -> Self {
        self.selectable = false;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn data(mut self, data: impl Into<String>) -> Self {
        self.data = Some(data.into());
        self
    }

    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

/// Tree style variants
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TreeStyle {
    /// Default with lines
    #[default]
    Default,
    /// No connecting lines
    NoLines,
    /// Indented only
    Indented,
}

/// Events emitted by Tree
#[derive(Debug, Clone)]
pub enum TreeEvent {
    /// Node selected
    Selected(String),
    /// Node expanded
    Expanded(String),
    /// Node collapsed
    Collapsed(String),
    /// Node double-clicked
    DoubleClick(String),
}

/// File tree item for stateless rendering
#[derive(Clone)]
pub struct FileTreeItem {
    pub(crate) name: String,
    pub(crate) is_dir: bool,
    pub(crate) depth: usize,
    pub(crate) expanded: bool,
}

impl FileTreeItem {
    pub fn file(name: impl Into<String>, depth: usize) -> Self {
        Self {
            name: name.into(),
            is_dir: false,
            depth,
            expanded: false,
        }
    }

    pub fn dir(name: impl Into<String>, depth: usize, expanded: bool) -> Self {
        Self {
            name: name.into(),
            is_dir: true,
            depth,
            expanded,
        }
    }
}

/// Directory entry for listing
#[derive(Clone)]
pub struct DirectoryEntry {
    pub(crate) name: String,
    pub(crate) is_dir: bool,
    pub(crate) size: Option<String>,
    pub(crate) modified: Option<String>,
}

impl DirectoryEntry {
    pub fn file(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_dir: false,
            size: None,
            modified: None,
        }
    }

    pub fn dir(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_dir: true,
            size: None,
            modified: None,
        }
    }

    pub fn size(mut self, size: impl Into<String>) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn modified(mut self, modified: impl Into<String>) -> Self {
        self.modified = Some(modified.into());
        self
    }
}
