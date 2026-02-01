//! Language configuration

use std::sync::OnceLock;
use tree_sitter::Language;

/// Language configuration for query compilation
pub struct LanguageConfig {
    /// Language name
    pub name: &'static str,
    /// Aliases for language detection
    pub aliases: &'static [&'static str],
    /// Tree-sitter language
    pub language: Language,
    /// Highlight query source
    pub highlights: &'static str,
    /// Optional injections query (for embedded languages)
    pub injections: Option<&'static str>,
    /// Optional locals query (for local variable highlighting)
    pub locals: Option<&'static str>,
}

// Highlight queries (same as in highlighter.rs but kept here for modularity)
const RUST_HIGHLIGHTS: &str = include_str!("../queries/rust.scm");
const JS_HIGHLIGHTS: &str = include_str!("../queries/javascript.scm");
const TS_HIGHLIGHTS: &str = include_str!("../queries/typescript.scm");
const PYTHON_HIGHLIGHTS: &str = include_str!("../queries/python.scm");
const JSON_HIGHLIGHTS: &str = include_str!("../queries/json.scm");
const TOML_HIGHLIGHTS: &str = include_str!("../queries/toml.scm");
const BASH_HIGHLIGHTS: &str = include_str!("../queries/bash.scm");

// Language configurations - lazily initialized
static LANGUAGE_CONFIGS: OnceLock<Vec<LanguageConfig>> = OnceLock::new();

pub(crate) fn language_configs() -> &'static [LanguageConfig] {
    LANGUAGE_CONFIGS.get_or_init(|| vec![
        LanguageConfig {
            name: "rust",
            aliases: &["rs"],
            language: tree_sitter_rust::LANGUAGE.into(),
            highlights: RUST_HIGHLIGHTS,
            injections: None,
            locals: None,
        },
        LanguageConfig {
            name: "javascript",
            aliases: &["js", "jsx", "mjs", "cjs"],
            language: tree_sitter_javascript::LANGUAGE.into(),
            highlights: JS_HIGHLIGHTS,
            injections: None,
            locals: None,
        },
        LanguageConfig {
            name: "typescript",
            aliases: &["ts"],
            language: tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            highlights: TS_HIGHLIGHTS,
            injections: None,
            locals: None,
        },
        LanguageConfig {
            name: "tsx",
            aliases: &[],
            language: tree_sitter_typescript::LANGUAGE_TSX.into(),
            highlights: TS_HIGHLIGHTS,
            injections: None,
            locals: None,
        },
        LanguageConfig {
            name: "python",
            aliases: &["py", "pyw", "pyi"],
            language: tree_sitter_python::LANGUAGE.into(),
            highlights: PYTHON_HIGHLIGHTS,
            injections: None,
            locals: None,
        },
        LanguageConfig {
            name: "json",
            aliases: &["jsonc"],
            language: tree_sitter_json::LANGUAGE.into(),
            highlights: JSON_HIGHLIGHTS,
            injections: None,
            locals: None,
        },
        LanguageConfig {
            name: "toml",
            aliases: &[],
            language: tree_sitter_toml_ng::LANGUAGE.into(),
            highlights: TOML_HIGHLIGHTS,
            injections: None,
            locals: None,
        },
        LanguageConfig {
            name: "bash",
            aliases: &["sh", "shell", "zsh"],
            language: tree_sitter_bash::LANGUAGE.into(),
            highlights: BASH_HIGHLIGHTS,
            injections: None,
            locals: None,
        },
    ])
}
