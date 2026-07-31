; Highlights for RouterOS .rsc
;
; Ordering matters: later patterns win in tree-sitter, so the generic
; identifier rules come first and the specific ones override them.

; --- generic fallbacks -------------------------------------------------------

(identifier) @variable
; Addresses, ranges and unit values: 10.0.0.1/24, 1d12h..2d, 5180-5240:20
(bare_word) @constant

; --- structure ---------------------------------------------------------------

(section path: (path) @function)

(command name: (identifier) @keyword)

(script_statement name: (script_command) @keyword)

(call name: (variable) @function.call)

; --- properties --------------------------------------------------------------

(property key: (identifier) @property)
(property key: (nested_key) @property)
(property key: (string) @property)

; --- literals ----------------------------------------------------------------

(string) @string
(escape_sequence) @string.escape
(number) @number
(variable) @variable.builtin
(comment) @comment

; --- punctuation and operators -----------------------------------------------

(operator) @operator

"=" @operator
";" @punctuation.delimiter

[
  "["
  "]"
  "{"
  "}"
  "("
  ")"
] @punctuation.bracket

; --- well-known values -------------------------------------------------------
; RouterOS has no real booleans, but yes/no reads as one everywhere.

((identifier) @boolean
 (#match? @boolean "^(yes|no|true|false)$"))
