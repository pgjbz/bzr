# Bzr lang

A simple lang, maybe compiled, maybe interpreted

Implements: <br>
- [x] Lexer

Tokens:
- Illegal(Location) => Illegal token 
- EOF(Location) => End of lile 
- Ident(String, Location) => Identifier like`variable_name` 
- Number(String, Location) => Number value like> 12 345 
- Assign(Location) => `=` 
- Plus(Location) => `+` 
- Comma(Location) => `,` 
- Semicolon(Location) => `;` 
- Lparen(Location) => `(` 
- Rparen(Location) => `)` 
- Lbrace(Location) => `{` 
- Rbrace(Location) => `}` 
- String(String, Location) = "like> this" 
- Function(Location) =>  `fn` 
- Let(Location) = `let` (i will make this variable> immutable) 
- Var(Location) = `var` (i will make this variable> mutable) 
- True(Location) = boolean value: `true` 
- False(Location) = boolean value: `false` 
- If(Location) => `if` 
- Else(Location) =>  `else` 
- Return(Location) =>  `ret` 
- Int(Location) =>  `int` 
- Str(Location) =>  `str` 
- Minus(Location) => `-` 
- Bang(Location) => `!` 
- Asterisk(Location) => `*` 
- Slash(Location) => `/` 
- Lt(Location) => `<` 
- Gt(Location) => `<` 

```rust
struct Location {
    line: usize,
    position: usize,
    filename: &'static str
}
```