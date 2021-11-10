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
            Self::And(_) => {
                format!("&&")
            },
            Self::Assign(_) => {
                format!("=")
            },
            Self::Asterisk(_) => {
                format!("*")
            },
            Self::Bang(_) => {
                format!("!")
            },
            Self::Bool(_) => {
                format!("boolean type")
            },
            Self::Comma(_) => {
                format!(",")
            },
            Self::Diff(_) => {
                format!("!=")
            },
            Self::EOF(_) => {
                format!("EOF")
            },
            Self::Else(_) => {
                format!("else")
            },
            Self::Eq(_) => {
                format!("==")
            },
            Self::Function(_) => {
                format!("fn")
            },
            Self::Ident(_, _) => {
                format!("identifier")
            },
            Self::Int(_) => {
                format!("integer type")
            },
            Self::LSqBracket(_) => {
                format!("[")
            },
            Self::Lbrace(_) => {
                format!("{{")
            },
            Self::Let(_) => {
                format!("let")
            },
            Self::Lparen(_) => {
                format!("(")
            },
            Self::Number(_, _) => {
                format!("number value")
            },
            Self::RSqBracket(_) => {
                format!("]")
            },
            Self::Rbrace(_) => {
                format!("}}")
            },
            Self::Return(_) => {
                format!("ret")
            },
            Self::Rparen(_) => {
                format!(")")
            },
            Self::Semicolon(_) => {
                format!(";")
            },
            Self::Str(_) => {
                format!("string type")
            },
            Self::String(_, _) => {
                format!("String value")
            },
            Self::Var(_) => {
                format!("var")
            },
            Self::While(_) => {
                format!("while")
            },
            _ => String::from("another thing, found it")
        };
        write!(f, "{}", str)
    }
}
