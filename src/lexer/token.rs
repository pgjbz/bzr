use std::{fmt::Display, rc::Rc};

#[derive(PartialEq, Debug)]
pub enum Token {
    Illegal(Option<Rc<String>>, Option<Location>),
    EOF(Option<Location>),
    Ident(Option<Rc<String>>, Option<Location>),
    Number(Option<Rc<String>>, Option<Location>),
    Comma(Option<Location>),
    Semicolon(Option<Location>),
    Lparen(Option<Location>),
    Rparen(Option<Location>),
    LSqBracket(Option<Location>),
    RSqBracket(Option<Location>),
    Lbrace(Option<Location>),
    Rbrace(Option<Location>),
    String(Option<Rc<String>>, Option<Location>),
    Function(Option<Location>),
    Let(Option<Location>),
    Var(Option<Location>),
    Bool(Option<Location>),
    True(Option<Location>),
    False(Option<Location>),
    While(Option<Location>),
    If(Option<Location>),
    Else(Option<Location>),
    Return(Option<Location>),
    Int(Option<Location>),
    Str(Option<Location>),
    Bang(Option<Location>),
    Asterisk(Option<Location>),
    Plus(Option<Location>),
    Minus(Option<Location>),
    Slash(Option<Location>),
    Assign(Option<Location>),
    Lt(Option<Location>),
    Gt(Option<Location>),
    Eq(Option<Location>),
    Lte(Option<Location>),
    Gte(Option<Location>),
    Diff(Option<Location>),
    And(Option<Location>),
    Or(Option<Location>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Location {
    pub position: usize,
    pub line: usize,
    filename: Rc<String>,
}

impl Location {
    pub fn new(position: usize, line: usize, filename: Rc<String>) -> Self {
        Self {
            position,
            line,
            filename,
        }
    }
}

impl Token {
    pub fn get_keyword_token(ident: &str, location: Option<Location>) -> Result<Token, String> {
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

impl Display for Token {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::And(_) => "&&".to_string(),
            Self::Assign(_) => "=".to_string(),
            Self::Asterisk(_) => "*".to_string(),
            Self::Bang(_) => "!".to_string(),
            Self::Bool(_) => "boolean type".to_string(),
            Self::Comma(_) => ",".to_string(),
            Self::Diff(_) => "!=".to_string(),
            Self::EOF(_) => "EOF".to_string(),
            Self::Else(_) => "else".to_string(),
            Self::Eq(_) => "==".to_string(),
            Self::Function(_) => "fn".to_string(),
            Self::Ident(_, _) => "identifier".to_string(),
            Self::Int(_) => "integer type".to_string(),
            Self::LSqBracket(_) => "[".to_string(),
            Self::Lbrace(_) => "{{".to_string(),
            Self::Let(_) => "let".to_string(),
            Self::Lparen(_) => "(".to_string(),
            Self::Number(_, _) => "number value".to_string(),
            Self::RSqBracket(_) => "]".to_string(),
            Self::Rbrace(_) => "}}".to_string(),
            Self::Return(_) => "ret".to_string(),
            Self::Rparen(_) => ")".to_string(),
            Self::Semicolon(_) => ";".to_string(),
            Self::Str(_) => "string type".to_string(),
            Self::String(_, _) => "String value".to_string(),
            Self::Var(_) => "var".to_string(),
            Self::While(_) => "while".to_string(),
            _ => String::from("another thing, found it")
        };
        write!(f, "{}", str)
    }
}
