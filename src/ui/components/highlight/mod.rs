//! Text highlighting and marking components

mod highlight;
mod mark;
mod multi_highlight;
mod search_match;
mod strikethrough;
mod text_diff;
mod types;

// Re-export types
pub use highlight::Highlight;
pub use mark::Mark;
pub use multi_highlight::MultiHighlight;
pub use search_match::SearchMatch;
pub use strikethrough::Strikethrough;
pub use text_diff::TextDiff;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight() {
        let hl = Highlight::new("important")
            .color(HighlightColor::Yellow)
            .style(TextHighlightStyle::Background);

        assert_eq!(hl.text, "important");
        assert_eq!(hl.color, HighlightColor::Yellow);
    }

    #[test]
    fn test_mark() {
        let mark = Mark::new("noted text")
            .annotate("See comment")
            .color(HighlightColor::Blue);

        assert_eq!(mark.text, "noted text");
        assert!(mark.annotation.is_some());
    }

    #[test]
    fn test_strikethrough() {
        let strike = Strikethrough::new("old").replaced_with("new");

        assert_eq!(strike.text, "old");
        assert_eq!(strike.replacement, Some("new".to_string()));
    }

    #[test]
    fn test_search_match_finding() {
        let sm = SearchMatch::new("hello world hello", "hello");
        let matches = sm.find_matches();

        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0], (0, 5));
        assert_eq!(matches[1], (12, 17));
    }

    #[test]
    fn test_multi_highlight() {
        let mh = MultiHighlight::new("This is a test string")
            .highlight(0, 4, HighlightColor::Yellow)
            .highlight(10, 14, HighlightColor::Green);

        assert_eq!(mh.highlights.len(), 2);
    }
}
