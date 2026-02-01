//! Mention Parser
//!
//! Parses @file, @snippet, @url mentions in chat input.

mod types;
mod parser;
mod utils;

pub use types::{Mention, MentionKind, PartialMention, PartialMentionKind};
pub use parser::{parse_mentions, get_mention_at_cursor};

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_file_mention() {
        let mentions = parse_mentions("Look at @file:src/main.rs please");
        assert_eq!(mentions.len(), 1);
        assert_eq!(
            mentions[0].kind,
            MentionKind::File(PathBuf::from("src/main.rs"))
        );
    }

    #[test]
    fn test_parse_implicit_file() {
        let mentions = parse_mentions("Check @./src/lib.rs");
        assert_eq!(mentions.len(), 1);
        assert_eq!(
            mentions[0].kind,
            MentionKind::File(PathBuf::from("./src/lib.rs"))
        );
    }

    #[test]
    fn test_parse_file_with_line_range() {
        let mentions = parse_mentions("See @file:src/main.rs:10-20");
        assert_eq!(mentions.len(), 1);
        match &mentions[0].kind {
            MentionKind::FileRange {
                path,
                start_line,
                end_line,
            } => {
                assert_eq!(path, &PathBuf::from("src/main.rs"));
                assert_eq!(*start_line, 10);
                assert_eq!(*end_line, Some(20));
            }
            _ => panic!("Expected FileRange"),
        }
    }

    #[test]
    fn test_parse_snippet_mention() {
        let mentions = parse_mentions("Use @snippet:auth_handler");
        assert_eq!(mentions.len(), 1);
        assert_eq!(
            mentions[0].kind,
            MentionKind::Snippet("auth_handler".to_string())
        );
    }

    #[test]
    fn test_parse_url_mention() {
        let mentions = parse_mentions("See @url:https://example.com/docs");
        assert_eq!(mentions.len(), 1);
        assert_eq!(
            mentions[0].kind,
            MentionKind::Url("https://example.com/docs".to_string())
        );
    }

    #[test]
    fn test_parse_multiple_mentions() {
        let mentions = parse_mentions("Compare @file:a.rs with @file:b.rs");
        assert_eq!(mentions.len(), 2);
    }

    #[test]
    fn test_partial_mention() {
        let partial = get_mention_at_cursor("Look at @file:src/ma", 20);
        assert!(partial.is_some());
        let p = partial.unwrap();
        assert!(matches!(p.kind, PartialMentionKind::File));
        assert_eq!(p.prefix, "file:src/ma");
    }
}
