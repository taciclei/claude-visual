//! Model Selector Component
//!
//! UI component for selecting AI model and provider.

mod colors;
mod core;
mod render;
mod traits;
mod types;

pub use core::ModelSelector;
pub use types::{ModelCategory, ModelSelectorEvent, ProviderInfo};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_category() {
        assert_eq!(ModelCategory::Cloud, ModelCategory::Cloud);
        assert_ne!(ModelCategory::Cloud, ModelCategory::Local);
    }
}
