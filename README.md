# Bzr lang

A simple lang, maybe compiled, maybe interpreted

Implements: <br>
- [x] Lexer

Tokens:
- Illegal(usize) = Illegal token | position
- EOF(usize) = End of file | position
- Ident(String, usize) = Identifier like `variable_name` | position
- Number(String, usize) = Number value like 12345 | position
- Assign(char, usize) = `=` | position
- Plus(char, usize) = `+` | position
- Comma(char, usize) = `,` | position
- Semicolon(char, usize) = `;` | position
- Lparen(char, usize) = `(` | position
- Rparen(char, usize) = `)` | position
- Lbrace(char, usize) = `{` | position
- Rbrace(char, usize) = `}` | position
- String(String, usize) = "like this" | position
- Function(usize) = `fn` | position
- Let(usize) = `let` (i will make this variable immutable) | position
- Var(usize) = `var` (i will make this variable mutable) | position
- True(usize) = boolean value `true`, | position
- False(usize) = boolean value `false`, | position
- If(usize) = `if`, | position
- Else(usize) = `else`, | position
- Return(usize) = `ret`, | position
- Int(usize) = `int`, | position
- Str(usize) = `str`, | position
- Minus(char, usize) = `-`, | position
- Bang(char, usize) = `!`, | position
- Asterisk(char, usize) = `*`, | position
- Slash(char, usize) = `/`, | position
- Lt(char, usize) = `<`, | position
- Gt(char, usize) = `<` | position