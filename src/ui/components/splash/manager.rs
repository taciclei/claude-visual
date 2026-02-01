//! Splash screen manager for controlling startup flow

use std::time::{Duration, Instant};

/// Splash screen manager for controlling startup flow
pub struct SplashManager {
    /// Whether to show splash screen
    pub show_splash: bool,
    /// Minimum display time (ms)
    pub min_display_time: u64,
    /// Start time
    start_time: Option<Instant>,
}

impl SplashManager {
    /// Create a new splash manager
    pub fn new() -> Self {
        Self {
            show_splash: true,
            min_display_time: 500, // Show for at least 500ms
            start_time: None,
        }
    }

    /// Start the splash screen timer
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    /// Check if minimum time has elapsed
    pub fn can_dismiss(&self) -> bool {
        match self.start_time {
            Some(start) => start.elapsed() >= Duration::from_millis(self.min_display_time),
            None => true,
        }
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Option<Duration> {
        self.start_time.map(|s| s.elapsed())
    }

    /// Dismiss the splash screen
    pub fn dismiss(&mut self) {
        self.show_splash = false;
    }
}

impl Default for SplashManager {
    fn default() -> Self {
        Self::new()
    }
}
