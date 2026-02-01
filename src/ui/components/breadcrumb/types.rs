//! Types for breadcrumb component

/// A single breadcrumb item
#[derive(Clone)]
pub struct BreadcrumbItem {
    /// Display label
    pub label: String,
    /// Optional icon (emoji or symbol)
    pub icon: Option<String>,
    /// Whether this item is clickable
    pub clickable: bool,
    /// Unique identifier for the item
    pub id: String,
}

impl BreadcrumbItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            clickable: true,
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn non_clickable(mut self) -> Self {
        self.clickable = false;
        self
    }
}

/// Events emitted by Breadcrumb
pub enum BreadcrumbEvent {
    /// An item was clicked
    ItemClicked(String),
}
