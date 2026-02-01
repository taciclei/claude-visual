//! Context menu types and enums

/// Context menu item type
#[derive(Debug, Clone, PartialEq)]
pub enum ContextMenuItemType {
    /// Regular action item
    Action,
    /// Submenu with children
    Submenu,
    /// Checkbox item
    Checkbox(bool),
    /// Radio item in a group
    Radio { group: String, selected: bool },
    /// Separator line
    Separator,
}

/// Events emitted by context menu
#[derive(Debug, Clone)]
pub enum ContextMenuEvent {
    /// Item clicked
    ItemClicked(String),
    /// Checkbox toggled
    CheckboxToggled { id: String, checked: bool },
    /// Radio selected
    RadioSelected { id: String, group: String },
    /// Menu closed
    Closed,
}

/// Context menu item (renamed to avoid conflict with gpui::MenuItem)
#[derive(Clone)]
pub struct ContextMenuItem {
    /// Unique ID
    pub id: String,
    /// Display label
    pub label: String,
    /// Icon (emoji or symbol)
    pub icon: Option<String>,
    /// Keyboard shortcut display
    pub shortcut: Option<String>,
    /// Item type
    pub item_type: ContextMenuItemType,
    /// Whether disabled
    pub disabled: bool,
    /// Danger/destructive action
    pub danger: bool,
    /// Submenu items
    pub children: Vec<ContextMenuItem>,
}

impl ContextMenuItem {
    pub fn action(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            shortcut: None,
            item_type: ContextMenuItemType::Action,
            disabled: false,
            danger: false,
            children: Vec::new(),
        }
    }

    pub fn submenu(
        id: impl Into<String>,
        label: impl Into<String>,
        children: Vec<ContextMenuItem>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            shortcut: None,
            item_type: ContextMenuItemType::Submenu,
            disabled: false,
            danger: false,
            children,
        }
    }

    pub fn checkbox(id: impl Into<String>, label: impl Into<String>, checked: bool) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            shortcut: None,
            item_type: ContextMenuItemType::Checkbox(checked),
            disabled: false,
            danger: false,
            children: Vec::new(),
        }
    }

    pub fn radio(
        id: impl Into<String>,
        label: impl Into<String>,
        group: impl Into<String>,
        selected: bool,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            shortcut: None,
            item_type: ContextMenuItemType::Radio {
                group: group.into(),
                selected,
            },
            disabled: false,
            danger: false,
            children: Vec::new(),
        }
    }

    pub fn separator() -> Self {
        Self {
            id: "separator".to_string(),
            label: String::new(),
            icon: None,
            shortcut: None,
            item_type: ContextMenuItemType::Separator,
            disabled: false,
            danger: false,
            children: Vec::new(),
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn danger(mut self) -> Self {
        self.danger = true;
        self
    }
}
