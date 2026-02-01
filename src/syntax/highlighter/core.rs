//! Core highlighter implementation

use std::collections::HashMap;

use streaming_iterator::StreamingIterator as _;
use tree_sitter::{Language, Parser, Query, QueryCursor};

use crate::app::theme::SyntaxColors;

use super::queries::*;
use super::types::HighlightedSpan;
use super::utils::{build_spans, capture_name_to_color};

/// Syntax highlighter using tree-sitter
pub struct Highlighter {
    pub(crate) parsers: HashMap<String, Parser>,
    pub(crate) queries: HashMap<String, Query>,
}

impl Highlighter {
    /// Create a new highlighter with all supported languages
    pub fn new() -> Self {
        let mut parsers = HashMap::new();
        let mut queries = HashMap::new();

        // Initialize parsers for each language
        Self::init_language(&mut parsers, &mut queries, "rust", tree_sitter_rust::LANGUAGE.into(), RUST_HIGHLIGHTS);
        Self::init_language(&mut parsers, &mut queries, "javascript", tree_sitter_javascript::LANGUAGE.into(), JS_HIGHLIGHTS);
        Self::init_language(&mut parsers, &mut queries, "typescript", tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(), TS_HIGHLIGHTS);
        Self::init_language(&mut parsers, &mut queries, "tsx", tree_sitter_typescript::LANGUAGE_TSX.into(), TS_HIGHLIGHTS);
        Self::init_language(&mut parsers, &mut queries, "python", tree_sitter_python::LANGUAGE.into(), PYTHON_HIGHLIGHTS);
        Self::init_language(&mut parsers, &mut queries, "json", tree_sitter_json::LANGUAGE.into(), JSON_HIGHLIGHTS);
        Self::init_language(&mut parsers, &mut queries, "toml", tree_sitter_toml_ng::LANGUAGE.into(), TOML_HIGHLIGHTS);
        Self::init_language(&mut parsers, &mut queries, "bash", tree_sitter_bash::LANGUAGE.into(), BASH_HIGHLIGHTS);
        Self::init_language(&mut parsers, &mut queries, "sh", tree_sitter_bash::LANGUAGE.into(), BASH_HIGHLIGHTS);
        Self::init_language(&mut parsers, &mut queries, "shell", tree_sitter_bash::LANGUAGE.into(), BASH_HIGHLIGHTS);

        Self { parsers, queries }
    }

    fn init_language(
        parsers: &mut HashMap<String, Parser>,
        queries: &mut HashMap<String, Query>,
        name: &str,
        language: Language,
        highlight_query: &str,
    ) {
        let mut parser = Parser::new();
        if parser.set_language(&language).is_ok() {
            parsers.insert(name.to_string(), parser);

            if let Ok(query) = Query::new(&language, highlight_query) {
                queries.insert(name.to_string(), query);
            }
        }
    }

    /// Highlight code and return styled spans for a single line
    pub fn highlight_line(
        &mut self,
        code: &str,
        language: Option<&str>,
        syntax_colors: &SyntaxColors,
    ) -> Vec<HighlightedSpan> {
        let lang = language.map(|l| l.to_lowercase());
        let lang_key = lang.as_deref().unwrap_or("");

        // Try to get parser and query for this language
        let (parser, query) = match (self.parsers.get_mut(lang_key), self.queries.get(lang_key)) {
            (Some(p), Some(q)) => (p, q),
            _ => {
                // No highlighting available, return as plain text
                return vec![HighlightedSpan {
                    text: code.to_string(),
                    color: None,
                }];
            }
        };

        // Parse the code
        let tree = match parser.parse(code, None) {
            Some(t) => t,
            None => {
                return vec![HighlightedSpan {
                    text: code.to_string(),
                    color: None,
                }];
            }
        };

        // Execute highlight query
        let mut cursor = QueryCursor::new();

        // Collect all highlight spans
        let mut highlights: Vec<(usize, usize, gpui::Hsla)> = Vec::new();

        let mut matches = cursor.matches(query, tree.root_node(), code.as_bytes());
        while let Some(match_) = matches.next() {
            for capture in match_.captures {
                let capture_name = &query.capture_names()[capture.index as usize];
                let node = capture.node;
                let start = node.start_byte();
                let end = node.end_byte();

                if let Some(color) = capture_name_to_color(capture_name, syntax_colors) {
                    highlights.push((start, end, color));
                }
            }
        }

        // Sort highlights by start position
        highlights.sort_by_key(|(start, _, _)| *start);

        // Convert to spans
        build_spans(code, &highlights)
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}
