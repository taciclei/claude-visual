//! Workspace types

/// Sidebar tab selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarTab {
    Projects,
    Files,
    History,
    Git,
    Team,
}
