//! Empty state component for when there's no data

mod placeholder;
mod state;
mod types;

pub use placeholder::*;
pub use state::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_state_size() {
        assert_eq!(EmptyStateSize::Small.icon_size(), 32.0);
        assert_eq!(EmptyStateSize::Medium.icon_size(), 48.0);
        assert_eq!(EmptyStateSize::Large.icon_size(), 64.0);
    }

    #[test]
    fn test_empty_placeholder() {
        let placeholder = EmptyPlaceholder::no_results();
        assert_eq!(placeholder.icon, "🔍");
        assert_eq!(placeholder.message, "No results found");
    }
}
