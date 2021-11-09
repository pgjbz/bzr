#[derive(PartialEq, Debug)]
pub enum Token<'a> {
    STF,
    Illegal(String, Location<'a>),
    EOF(Location<'a>),
    Ident(String, Location<'a>),
    Number(String, Location<'a>),
    Assign(Location<'a>),
    Plus(Location<'a>),
    Comma(Location<'a>),
    Semicolon(Location<'a>),
    Lparen(Location<'a>),
    Rparen(Location<'a>),
    Lbrace(Location<'a>),
    Rbrace(Location<'a>),
    String(String, Location<'a>),
    Function(Location<'a>),
    Let(Location<'a>),
    Var(Location<'a>),
    True(Location<'a>),
    False(Location<'a>),
    If(Location<'a>),
    Else(Location<'a>),
    Return(Location<'a>),
    Int(Location<'a>),
    Str(Location<'a>),
    Bool(Location<'a>),
    Minus(Location<'a>),
    Bang(Location<'a>),
    Asterisk(Location<'a>),
    Slash(Location<'a>),
    Lt(Location<'a>),
    Gt(Location<'a>),
    Eq(Location<'a>),
    Lte(Location<'a>),
    Gte(Location<'a>),
    And(Location<'a>),
    Diff(Location<'a>),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Location<'a> {
    position: usize,
    line: usize,
    filename: &'a str,
}

impl<'a> Location<'a> {
    pub fn new(position: usize, line: usize, filename: &'a str) -> Self {
        Self {
            position,
            line,
            filename,
        }
    }
}

impl<'a> Token<'a> {
    pub fn get_keyword_token(ident: &str, location: Location<'a>) -> Result<Token<'a>, String> {
        let identifier: String = String::from(ident);
        match &identifier[..] {
            "if" => Ok(Token::If(location)),
            "else" => Ok(Token::Else(location)),
            "fn" => Ok(Token::Function(location)),
            "let" => Ok(Token::Let(location)),
            "var" => Ok(Token::Var(location)),
            "ret" => Ok(Token::Return(location)),
            "true" => Ok(Token::True(location)),
            "false" => Ok(Token::False(location)),
            "int" => Ok(Token::Int(location)),
            "str" => Ok(Token::Str(location)),
            "bool" => Ok(Token::Bool(location)),
            _ => Err(String::from("Not a keyword")),
        }
    }
}
