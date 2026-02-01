; TypeScript highlight queries

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
"implements" @keyword
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
"interface" @keyword
"type" @keyword
"enum" @keyword
"namespace" @keyword
"declare" @keyword
"abstract" @keyword
"public" @keyword
"private" @keyword
"protected" @keyword
"readonly" @keyword
"static" @keyword
"as" @keyword

(string) @string
(template_string) @string

(number) @number

(true) @constant
(false) @constant
(null) @constant
(undefined) @constant

(type_identifier) @type
(predefined_type) @type.builtin

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
"<" @punctuation.bracket
">" @punctuation.bracket

";" @punctuation.delimiter
"," @punctuation.delimiter
"." @punctuation.delimiter
":" @punctuation.delimiter

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
