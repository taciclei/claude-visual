//! Splash Screen Component
//!
//! Shows a loading screen during application startup.

mod manager;
mod screen;
mod types;

pub use manager::SplashManager;
pub use screen::SplashScreen;
pub use types::{SplashEvent, StartupPhase};
