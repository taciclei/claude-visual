//! TOML tree-sitter highlight queries

pub const TOML_HIGHLIGHTS: &str = r#"
(comment) @comment

(string) @string
(integer) @number
(float) @number
(boolean) @constant

"{" @punctuation.bracket
"}" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket

"=" @operator
"." @punctuation.delimiter
"," @punctuation.delimiter

(bare_key) @property
(dotted_key) @property
(table (bare_key) @type)
(table_array_element (bare_key) @type)
"#;
