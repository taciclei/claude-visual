//! Shared types for progress components

/// Progress bar style
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ProgressStyle {
    /// Standard progress bar
    #[default]
    Default,
    /// Thin progress bar (like loading indicators)
    Thin,
    /// Rounded ends
    Rounded,
    /// Striped pattern (animated)
    Striped,
}

/// Progress bar color scheme
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ProgressColor {
    /// Use accent color
    #[default]
    Accent,
    /// Success (green)
    Success,
    /// Warning (yellow/orange)
    Warning,
    /// Error (red)
    Error,
    /// Custom (uses theme surface)
    Neutral,
}

/// Loading indicator size
#[derive(Debug, Clone, Copy, Default)]
pub enum LoadingSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Events emitted by ProgressBar
pub enum ProgressBarEvent {
    /// Progress completed
    Completed,
}
