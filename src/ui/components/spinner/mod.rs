//! Spinner and loading indicator components

mod types;
mod spinner;
mod loading_overlay;
mod skeleton_loader;
mod loading_button;
mod progress_spinner;
mod inline_loader;

pub use types::{SpinnerSize, SpinnerVariant};
pub use spinner::Spinner;
pub use loading_overlay::LoadingOverlay;
pub use skeleton_loader::SkeletonLoader;
pub use loading_button::LoadingButton;
pub use progress_spinner::ProgressSpinner;
pub use inline_loader::InlineLoader;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_sizes() {
        assert_eq!(SpinnerSize::Small.size(), 16.0);
        assert_eq!(SpinnerSize::Medium.size(), 24.0);
        assert_eq!(SpinnerSize::Custom(40.0).size(), 40.0);
    }

    #[test]
    fn test_spinner_builder() {
        let spinner = Spinner::new()
            .size(SpinnerSize::Large)
            .variant(SpinnerVariant::Dots)
            .label("Loading...");

        assert_eq!(spinner.size, SpinnerSize::Large);
        assert_eq!(spinner.variant, SpinnerVariant::Dots);
        assert_eq!(spinner.label, Some("Loading...".to_string()));
    }

    #[test]
    fn test_loading_button() {
        let btn = LoadingButton::new("Submit")
            .loading(true)
            .disabled(false);

        assert!(btn.is_loading);
        assert!(!btn.disabled);
    }

    #[test]
    fn test_progress_spinner() {
        let spinner = ProgressSpinner::new(75.0);
        assert_eq!(spinner.progress, 75.0);

        let clamped = ProgressSpinner::new(150.0);
        assert_eq!(clamped.progress, 100.0);
    }
}
