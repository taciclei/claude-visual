; Bash highlight queries

(comment) @comment

"if" @keyword
"then" @keyword
"else" @keyword
"elif" @keyword
"fi" @keyword
"for" @keyword
"in" @keyword
"do" @keyword
"done" @keyword
"while" @keyword
"until" @keyword
"case" @keyword
"esac" @keyword
"function" @keyword
"return" @keyword
"local" @keyword
"export" @keyword
"readonly" @keyword
"declare" @keyword

(string) @string
(raw_string) @string

(number) @number

(command_name) @function.call
(function_definition name: (word) @function)

(variable_name) @variable
(special_variable_name) @variable.builtin

"(" @punctuation.bracket
")" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"[[" @punctuation.bracket
"]]" @punctuation.bracket

";" @punctuation.delimiter
"|" @operator
">" @operator
"<" @operator
">>" @operator
"&&" @operator
"||" @operator
"=" @operator
"$" @operator
