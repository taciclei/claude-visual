//! Shared types for splash screen

/// Startup phase for progress indication
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupPhase {
    /// Initial loading
    Initializing,
    /// Loading configuration
    LoadingConfig,
    /// Connecting to database
    ConnectingDatabase,
    /// Loading theme
    LoadingTheme,
    /// Loading plugins
    LoadingPlugins,
    /// Starting services
    StartingServices,
    /// Ready to show main UI
    Ready,
}

impl StartupPhase {
    /// Get display text for the phase
    pub fn display_text(&self) -> &'static str {
        match self {
            StartupPhase::Initializing => "Initializing...",
            StartupPhase::LoadingConfig => "Loading configuration...",
            StartupPhase::ConnectingDatabase => "Connecting to database...",
            StartupPhase::LoadingTheme => "Loading theme...",
            StartupPhase::LoadingPlugins => "Loading plugins...",
            StartupPhase::StartingServices => "Starting services...",
            StartupPhase::Ready => "Ready",
        }
    }

    /// Get progress percentage (0.0 - 1.0)
    pub fn progress(&self) -> f32 {
        match self {
            StartupPhase::Initializing => 0.0,
            StartupPhase::LoadingConfig => 0.15,
            StartupPhase::ConnectingDatabase => 0.30,
            StartupPhase::LoadingTheme => 0.50,
            StartupPhase::LoadingPlugins => 0.70,
            StartupPhase::StartingServices => 0.85,
            StartupPhase::Ready => 1.0,
        }
    }

    /// Get the next phase
    pub fn next(&self) -> Option<Self> {
        match self {
            StartupPhase::Initializing => Some(StartupPhase::LoadingConfig),
            StartupPhase::LoadingConfig => Some(StartupPhase::ConnectingDatabase),
            StartupPhase::ConnectingDatabase => Some(StartupPhase::LoadingTheme),
            StartupPhase::LoadingTheme => Some(StartupPhase::LoadingPlugins),
            StartupPhase::LoadingPlugins => Some(StartupPhase::StartingServices),
            StartupPhase::StartingServices => Some(StartupPhase::Ready),
            StartupPhase::Ready => None,
        }
    }
}

/// Events from splash screen
#[derive(Debug, Clone)]
pub enum SplashEvent {
    /// Startup complete, ready to show main UI
    Complete,
    /// Error during startup
    Error(String),
}
