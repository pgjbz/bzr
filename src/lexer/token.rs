use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum Token<'a> {
    Illegal(Option<String>, Option<Location<'a>>),
    EOF(Option<Location<'a>>),
    Ident(Option<String>, Option<Location<'a>>),
    Number(Option<String>, Option<Location<'a>>),
    Comma(Option<Location<'a>>),
    Semicolon(Option<Location<'a>>),
    Lparen(Option<Location<'a>>),
    Rparen(Option<Location<'a>>),
    LSqBracket(Option<Location<'a>>),
    RSqBracket(Option<Location<'a>>),
    Lbrace(Option<Location<'a>>),
    Rbrace(Option<Location<'a>>),
    String(Option<String>, Option<Location<'a>>),
    Function(Option<Location<'a>>),
    Let(Option<Location<'a>>),
    Var(Option<Location<'a>>),
    Bool(Option<Location<'a>>),
    True(Option<Location<'a>>),
    False(Option<Location<'a>>),
    While(Option<Location<'a>>),
    If(Option<Location<'a>>),
    Else(Option<Location<'a>>),
    Return(Option<Location<'a>>),
    Int(Option<Location<'a>>),
    Str(Option<Location<'a>>),
    Bang(Option<Location<'a>>),
    Asterisk(Option<Location<'a>>),
    Plus(Option<Location<'a>>),
    Minus(Option<Location<'a>>),
    Slash(Option<Location<'a>>),
    Assign(Option<Location<'a>>),
    Lt(Option<Location<'a>>),
    Gt(Option<Location<'a>>),
    Eq(Option<Location<'a>>),
    Lte(Option<Location<'a>>),
    Gte(Option<Location<'a>>),
    Diff(Option<Location<'a>>),
    And(Option<Location<'a>>),
    Or(Option<Location<'a>>),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Location<'a> {
    pub position: usize,
    pub line: usize,
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
    pub fn get_keyword_token(ident: &str, location: Option<Location<'a>>) -> Result<Token<'a>, String> {
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
            "while" => Ok(Token::While(location)),
            _ => Err(String::from("Not a keyword")),
        }
    }
}

impl<'a> Display for Token<'a> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::And(loc) => {
                let loc = loc.unwrap();
                format!("&& in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::Assign(loc) => {
                let loc = loc.unwrap();
                format!("= in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::Asterisk(loc) => {
                let loc = loc.unwrap();
                format!("* in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::Bang(loc) => {
                let loc = loc.unwrap();
                format!("! in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::Bool(loc) => {
                let loc = loc.unwrap();
                format!("boolean type in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::Comma(loc) => {
                let loc = loc.unwrap();
                format!(", in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::Diff(loc) => {
                let loc = loc.unwrap();
                format!("!= in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::EOF(loc) => {
                let loc = loc.unwrap();
                format!("EOF in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::Else(loc) => {
                let loc = loc.unwrap();
                format!("else in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::Eq(loc) => {
                let loc = loc.unwrap();
                format!("== in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::Function(loc) => {
                let loc = loc.unwrap();
                format!("fn in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::Ident(_, loc) => {
                let loc = loc.unwrap();
                format!("identifier in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::Int(loc) => {
                let loc = loc.unwrap();
                format!("integer type in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::LSqBracket(loc) => {
                let loc = loc.unwrap();
                format!("[ in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::Lbrace(loc) => {
                let loc = loc.unwrap();
                format!("{{ in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::Let(loc) => {
                let loc = loc.unwrap();
                format!("let in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::Lparen(loc) => {
                let loc = loc.unwrap();
                format!("( in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::Number(_, loc) => {
                let loc = loc.unwrap();
                format!("number value in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::RSqBracket(loc) => {
                let loc = loc.unwrap();
                format!("] in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::Rbrace(loc) => {
                let loc = loc.unwrap();
                format!("}} in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::Return(loc) => {
                let loc = loc.unwrap();
                format!("ret in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::Rparen(loc) => {
                let loc = loc.unwrap();
                format!(") in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::Semicolon(loc) => {
                let loc = loc.unwrap();
                format!("; in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::Str(loc) => {
                let loc = loc.unwrap();
                format!("string type in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::String(_, loc) => {
                let loc = loc.unwrap();
                format!("String value in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::Var(loc) => {
                let loc = loc.unwrap();
                format!("var in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            Self::While(loc) => {
                let loc = loc.unwrap();
                format!("while in {}:{}:{}", loc.filename, loc.line, loc.position)
            },
            _ => String::from("another thing, found it")
        };
        write!(f, "{}", str)
    }
}
