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

For now only get the tokens, on input with the code above:

```
let numero int = 5;
if numero >= 4 {
	var st str = "manipulado";
} else {
	var st str = "sera?";
} 
```

Result in:

```
Let(Location { position: 1, line: 1, filename: "foo.bzr" })
Ident("numero", Location { position: 5, line: 1, filename: "foo.bzr" })
Int(Location { position: 12, line: 1, filename: "foo.bzr" })
Assign(Location { position: 16, line: 1, filename: "foo.bzr" })
Number("5", Location { position: 18, line: 1, filename: "foo.bzr" })
Semicolon(Location { position: 19, line: 1, filename: "foo.bzr" })
If(Location { position: 1, line: 2, filename: "foo.bzr" })
Ident("numero", Location { position: 4, line: 2, filename: "foo.bzr" })
Gte(Location { position: 11, line: 2, filename: "foo.bzr" })
Number("4", Location { position: 14, line: 2, filename: "foo.bzr" })
Lbrace(Location { position: 16, line: 2, filename: "foo.bzr" })
Var(Location { position: 2, line: 3, filename: "foo.bzr" })
Ident("st", Location { position: 6, line: 3, filename: "foo.bzr" })
Str(Location { position: 9, line: 3, filename: "foo.bzr" })
Assign(Location { position: 13, line: 3, filename: "foo.bzr" })
String("manipulado", Location { position: 15, line: 3, filename: "foo.bzr" })
Semicolon(Location { position: 27, line: 3, filename: "foo.bzr" })
Rbrace(Location { position: 1, line: 4, filename: "foo.bzr" })
Else(Location { position: 3, line: 4, filename: "foo.bzr" })
Lbrace(Location { position: 8, line: 4, filename: "foo.bzr" })
Var(Location { position: 2, line: 5, filename: "foo.bzr" })
Ident("st", Location { position: 6, line: 5, filename: "foo.bzr" })
Str(Location { position: 9, line: 5, filename: "foo.bzr" })
Assign(Location { position: 13, line: 5, filename: "foo.bzr" })
String("sera?", Location { position: 15, line: 5, filename: "foo.bzr" })
Semicolon(Location { position: 22, line: 5, filename: "foo.bzr" })
Rbrace(Location { position: 1, line: 6, filename: "foo.bzr" })
```

# Run lexer:

To run the lexer