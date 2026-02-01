//! Splash Screen Component
//!
//! Shows a loading screen during application startup.

mod types;
mod screen;
mod manager;

pub use types::{StartupPhase, SplashEvent};
pub use screen::SplashScreen;
pub use manager::SplashManager;
