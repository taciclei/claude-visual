//! Python tree-sitter highlight queries

pub const PYTHON_HIGHLIGHTS: &str = r##"
(comment) @comment

"def" @keyword
"class" @keyword
"if" @keyword
"elif" @keyword
"else" @keyword
"for" @keyword
"while" @keyword
"return" @keyword
"yield" @keyword
"raise" @keyword
"try" @keyword
"except" @keyword
"finally" @keyword
"with" @keyword
"as" @keyword
"import" @keyword
"from" @keyword
"pass" @keyword
"break" @keyword
"continue" @keyword
"lambda" @keyword
"and" @keyword
"or" @keyword
"not" @keyword
"in" @keyword
"is" @keyword
"global" @keyword
"nonlocal" @keyword
"async" @keyword
"await" @keyword

(string) @string
(interpolation) @string.special

(integer) @number
(float) @number

(true) @constant
(false) @constant
(none) @constant

(function_definition name: (identifier) @function)
(call function: (identifier) @function.call)
(call function: (attribute attribute: (identifier) @function.call))

(type) @type

(identifier) @variable
(attribute attribute: (identifier) @property)

"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket

":" @punctuation.delimiter
"," @punctuation.delimiter
"." @punctuation.delimiter

"=" @operator
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"@" @operator
"#"
"##;
