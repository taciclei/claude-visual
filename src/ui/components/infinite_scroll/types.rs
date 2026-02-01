//! Shared types for infinite scroll components

/// Scroll direction for infinite loading
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ScrollDirection {
    #[default]
    Down,
    Up,
    Both,
}

/// Loading state for infinite scroll
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LoadingState {
    #[default]
    Idle,
    Loading,
    LoadingMore,
    Error,
    EndReached,
}

/// Pull to refresh state
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PullState {
    #[default]
    Idle,
    Pulling,
    ReadyToRefresh,
    Refreshing,
}

/// Load more display variant
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LoadMoreVariant {
    #[default]
    Button,
    Auto,
    Link,
}
