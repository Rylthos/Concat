syn match  concatLineComment "\/\/.*"
syn match  concatNumber      /\<\d\+\(\.\d*\)\?\>/
syn region concatString      start=/"/ skip=/\\"/ end=/"/

syn keyword concatConditional if else
syn keyword concatLoop while
syn keyword concatType string bool i32 void
syn keyword concatBoolean true false
syn keyword concatKeyword rot3 dup drop over swap print cast func

hi def link concatLineComment Comment
hi def link concatLoop        Repeat
hi def link concatType        Type
hi def link concatNumber      Number
hi def link concatString      String
hi def link concatBoolean     Boolean
hi def link concatKeyword     Keyword
hi def link concatConditional Conditional
