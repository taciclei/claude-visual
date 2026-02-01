//! Menu types and shared structures

/// Menu item type
#[derive(Clone, Debug)]
pub enum MenuItemType {
    /// Regular clickable item
    Item,
    /// Separator line
    Separator,
    /// Submenu with nested items
    Submenu(Vec<MenuItemData>),
    /// Header/label (non-clickable)
    Header,
}

/// A single menu item (named to avoid conflict with gpui::MenuItem)
#[derive(Clone, Debug)]
pub struct MenuItemData {
    /// Item type
    pub item_type: MenuItemType,
    /// Unique identifier
    pub id: String,
    /// Display label
    pub label: String,
    /// Optional icon
    pub icon: Option<String>,
    /// Keyboard shortcut hint
    pub shortcut: Option<String>,
    /// Whether item is disabled
    pub disabled: bool,
    /// Whether item is destructive (shows in red)
    pub destructive: bool,
    /// Whether item is checked (for toggle items)
    pub checked: Option<bool>,
}

impl MenuItemData {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            item_type: MenuItemType::Item,
            id: id.into(),
            label: label.into(),
            icon: None,
            shortcut: None,
            disabled: false,
            destructive: false,
            checked: None,
        }
    }

    pub fn separator() -> Self {
        Self {
            item_type: MenuItemType::Separator,
            id: "separator".to_string(),
            label: String::new(),
            icon: None,
            shortcut: None,
            disabled: false,
            destructive: false,
            checked: None,
        }
    }

    pub fn header(label: impl Into<String>) -> Self {
        Self {
            item_type: MenuItemType::Header,
            id: "header".to_string(),
            label: label.into(),
            icon: None,
            shortcut: None,
            disabled: true,
            destructive: false,
            checked: None,
        }
    }

    pub fn submenu(id: impl Into<String>, label: impl Into<String>, items: Vec<MenuItemData>) -> Self {
        Self {
            item_type: MenuItemType::Submenu(items),
            id: id.into(),
            label: label.into(),
            icon: None,
            shortcut: None,
            disabled: false,
            destructive: false,
            checked: None,
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }
}

/// Events emitted by Menu
#[derive(Debug, Clone)]
pub enum MenuEvent {
    /// Item was selected
    Select(String),
    /// Menu was closed
    Close,
}

/// Events emitted by ContextMenu
#[derive(Debug, Clone)]
pub enum ContextMenuEvent {
    Select(String),
    Close,
}

/// Events emitted by ActionMenu
#[derive(Debug, Clone)]
pub enum ActionMenuEvent {
    Select(String),
}
