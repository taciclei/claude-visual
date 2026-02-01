//! Rust tree-sitter highlight queries

pub const RUST_HIGHLIGHTS: &str = r##"
(line_comment) @comment
(block_comment) @comment

"let" @keyword
"mut" @keyword
"const" @keyword
"static" @keyword
"if" @keyword
"else" @keyword
"match" @keyword
"for" @keyword
"while" @keyword
"loop" @keyword
"return" @keyword
"break" @keyword
"continue" @keyword
"fn" @keyword
"pub" @keyword
"mod" @keyword
"use" @keyword
"struct" @keyword
"enum" @keyword
"impl" @keyword
"trait" @keyword
"type" @keyword
"where" @keyword
"as" @keyword
"in" @keyword
"ref" @keyword
"self" @keyword
"Self" @keyword
"async" @keyword
"await" @keyword
"move" @keyword
"dyn" @keyword
"unsafe" @keyword
"extern" @keyword
"crate" @keyword
"super" @keyword

(string_literal) @string
(raw_string_literal) @string
(char_literal) @string

(integer_literal) @number
(float_literal) @number

(boolean_literal) @constant

(function_item name: (identifier) @function)
(call_expression function: (identifier) @function.call)
(call_expression function: (field_expression field: (field_identifier) @function.call))

(type_identifier) @type
(primitive_type) @type.builtin

(identifier) @variable
(field_identifier) @property

"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"<" @punctuation.bracket
">" @punctuation.bracket

";" @punctuation.delimiter
"," @punctuation.delimiter
"::" @punctuation.delimiter
":" @punctuation.delimiter
"." @punctuation.delimiter

"->" @operator
"=>" @operator
"=" @operator
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"&" @operator
"|" @operator
"!" @operator
"?" @operator
"#"
"##;
