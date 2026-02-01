//! Search input component

mod types;
mod search_input;
mod simple_search_bar;

pub use types::*;
pub use search_input::SearchInput;
pub use simple_search_bar::SimpleSearchBar;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_input_size() {
        assert_eq!(SearchInputSize::Small.height(), 28.0);
        assert_eq!(SearchInputSize::Medium.height(), 36.0);
        assert_eq!(SearchInputSize::Large.height(), 44.0);
    }

    #[test]
    fn test_simple_search_bar() {
        let bar = SimpleSearchBar::new("Search...")
            .with_query("test");

        assert_eq!(bar.placeholder, "Search...");
        assert_eq!(bar.query, "test");
    }
}
