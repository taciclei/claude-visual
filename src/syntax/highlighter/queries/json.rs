//! JSON tree-sitter highlight queries

pub const JSON_HIGHLIGHTS: &str = r#"
(string) @string
(number) @number
(true) @constant
(false) @constant
(null) @constant

"{" @punctuation.bracket
"}" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket

":" @punctuation.delimiter
"," @punctuation.delimiter

(pair key: (string) @property)
"#;
