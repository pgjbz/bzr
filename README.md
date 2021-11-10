# Bzr lang

A simple lang, maybe compiled, maybe interpreted

Implements: <br>
- [x] Lexer

Tokens:
- Illegal(Location) => Illegal token 
- EOF(Location) => End of lile 
- Ident(String, Location) => Identifier like`variable_name` 
- Number(String, Location) => Number value like> 12 345 
- Comma(Location) => `,` 
- Semicolon(Location) => `;` 
- Lparen(Location) => `(` 
- Rparen(Location) => `)` 
- Lbrace(Location) => `{` 
- Rbrace(Location) => `}` 
- String(String, Location) = "like> this" 
- Function(Location) =>  `fn` 
- Return(Location) => `ret` 
- Let(Location) = `let` (i will make this variable> immutable) 
- Var(Location) = `var` (i will make this variable> mutable)
- Bool(Location) = `bool` 
- Int(Location) => `int`  
- Str(Location) => `str` 
- True(Location) = boolean value: `true` 
- False(Location) = boolean value: `false` 
- While(Location) = `while`
- If(Location) => `if` 
- Else(Location) => `else` 
- Minus(Location) => `-` 
- Plus(Location) => `+` 
- Asterisk(Location) => `*` 
- Slash(Location) => `/` 
- Bang(Location) => `!` 
- Assign(Location) => `=` 
- Lt(Location) => `<` 
- Gt(Location) => `<` 
- Gte(Location) => `>=`
- Lte(Location) => `<=`
- Eq(Location) => `==`
- Diff(Location) => `!=`
- And(Location) => `&&`
- Or(Location) => `||`


```rust
struct Location {
    line: usize,
    position: usize,
    filename: &'static str
}
```

For now only get the tokens, on input with the code below:

```
let numero int = 5;
if numero >= 4 {
	var st str = "manipulado";
} else {
	var st str = "sera?";
} 
```

Result is:

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

To run the lexer use command: cargo r --release -- file.bzr


### Rust version to write this code:

Minival version used to write this code is [Rust 1.56](https://github.com/rust-lang/rust/releases/tag/1.56.0), but i think this work in others versions

<hr>
# Types

For now only have 3 types:

- Boolean => `bool`
- Integer => `int`
- String => `str`

<hr>
# Expressions

For now only parse 1 expressions in one line

<hr>
Let:<br>

```bzr
let name type = val
```
Examples:

Let type integer
```bzr
let number int = 10;
let another_number = 10;
```

Type inference is possible

Let type bool
```bzr
let boolean bool = false;
let another_boolean = true;
```

Let type String
```bzr
let string str = "text";
let another_string = "text";
```

Let error
```
let dife bool = 10;
```

<hr>