use gpui::*;

/// Navigation orientation
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NavOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Navigation size variants
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NavSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Navigation item variant
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NavItemVariant {
    #[default]
    Default,
    Subtle,
    Pill,
    Underline,
}

/// A navigation item
#[derive(Debug, Clone)]
pub struct NavItem {
    pub(crate) id: SharedString,
    pub(crate) label: SharedString,
    pub(crate) icon: Option<SharedString>,
    pub(crate) href: Option<SharedString>,
    pub(crate) active: bool,
    pub(crate) disabled: bool,
    pub(crate) badge: Option<SharedString>,
    pub(crate) children: Vec<NavItem>,
}

impl NavItem {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            href: None,
            active: false,
            disabled: false,
            badge: None,
            children: Vec::new(),
        }
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn href(mut self, href: impl Into<SharedString>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn badge(mut self, badge: impl Into<SharedString>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn children(mut self, children: Vec<NavItem>) -> Self {
        self.children = children;
        self
    }
}

/// A section in the sidebar navigation
#[derive(Debug, Clone)]
pub struct NavSection {
    pub(crate) title: Option<SharedString>,
    pub(crate) items: Vec<NavItem>,
}

impl NavSection {
    pub fn new() -> Self {
        Self {
            title: None,
            items: Vec::new(),
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn items(mut self, items: Vec<NavItem>) -> Self {
        self.items = items;
        self
    }
}

impl Default for NavSection {
    fn default() -> Self {
        Self::new()
    }
}
