//! JavaScript tree-sitter highlight queries

pub const JS_HIGHLIGHTS: &str = r##"
(comment) @comment

"const" @keyword
"let" @keyword
"var" @keyword
"function" @keyword
"return" @keyword
"if" @keyword
"else" @keyword
"for" @keyword
"while" @keyword
"do" @keyword
"switch" @keyword
"case" @keyword
"default" @keyword
"break" @keyword
"continue" @keyword
"throw" @keyword
"try" @keyword
"catch" @keyword
"finally" @keyword
"new" @keyword
"class" @keyword
"extends" @keyword
"import" @keyword
"export" @keyword
"from" @keyword
"async" @keyword
"await" @keyword
"yield" @keyword
"typeof" @keyword
"instanceof" @keyword
"in" @keyword
"of" @keyword
"this" @keyword
"super" @keyword

(string) @string
(template_string) @string

(number) @number

(true) @constant
(false) @constant
(null) @constant
(undefined) @constant

(function_declaration name: (identifier) @function)
(call_expression function: (identifier) @function.call)
(call_expression function: (member_expression property: (property_identifier) @function.call))

(identifier) @variable
(property_identifier) @property

"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket

";" @punctuation.delimiter
"," @punctuation.delimiter
"." @punctuation.delimiter

"=>" @operator
"=" @operator
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"&&" @operator
"||" @operator
"!" @operator
"?" @operator
":" @operator
"#"
"##;
