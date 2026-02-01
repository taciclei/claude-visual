//! Progress bar component

mod types;
mod progress_bar;
mod circular_progress;
mod loading_indicator;

pub use types::*;
pub use progress_bar::ProgressBar;
pub use circular_progress::CircularProgress;
pub use loading_indicator::LoadingIndicator;

#[cfg(test)]
mod tests {
    #[test]
    fn test_progress_clamp() {
        // Progress should be clamped between 0 and 1
        assert_eq!(1.5_f32.clamp(0.0, 1.0), 1.0);
        assert_eq!((-0.5_f32).clamp(0.0, 1.0), 0.0);
        assert_eq!(0.5_f32.clamp(0.0, 1.0), 0.5);
    }
}
