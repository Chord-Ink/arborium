; Slang adds and removes constructs from HLSL/C++; use its own node vocabulary.
(identifier) @variable
(field_identifier) @property
(type_identifier) @type
(primitive_type) @type
(sized_type_specifier) @type
(number_literal) @number
(string_literal) @string
(char_literal) @string
(raw_string_literal) @string
(comment) @comment
(true) @constant.builtin
(false) @constant.builtin
(null) @constant.builtin
["import" "interface" "extension" "let" "associatedtype" "struct" "class" "enum" "return" "if" "else" "for" "while" "do" "break" "continue" "switch" "case" "default" "public" "private" "static" "const" "typedef" "namespace" "using" "in" "out" "inout" "uniform" "typename" "where" "__init" "property" "get" "set"] @keyword
(function_declarator declarator: (identifier) @function)
(call_expression function: (identifier) @function)
