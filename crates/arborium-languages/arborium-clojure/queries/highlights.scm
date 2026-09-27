;; Literals

(num_lit) @number

[
  (char_lit)
  (str_lit)
] @string

[
 (bool_lit)
 (nil_lit)
] @constant.builtin

(kwd_lit) @constant

;; Comments

(comment) @comment

;; Treat quasiquotation as operators for the purpose of highlighting.

[
 "'"
 "`"
 "~"
 "@"
 "~@"
] @operator

;; Symbols, calls, and special forms (not just literal values).
(sym_lit) @variable
(list_lit . (sym_lit) @function)

((list_lit . (sym_lit) @keyword)
 (#any-of? @keyword "ns" "def" "defn" "defn-" "defmacro" "defmulti" "defmethod"
  "defrecord" "deftype" "defprotocol" "defonce" "let" "letfn" "if" "if-let" "if-not"
  "when" "when-let" "when-not" "cond" "condp" "case" "do" "doseq" "dotimes" "loop"
  "recur" "fn" "for" "try" "catch" "finally" "throw" "new" "set!" "quote" "var"))

((list_lit . (sym_lit) @_def . (sym_lit) @function)
 (#any-of? @_def "defn" "defn-" "defmacro" "defmulti" "defmethod"))

((list_lit . (sym_lit) @_ns . (sym_lit) @namespace)
 (#eq? @_ns "ns"))

["(" ")" "[" "]" "{" "}"] @punctuation.bracket
