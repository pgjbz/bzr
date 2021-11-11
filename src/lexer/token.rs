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
    pub filename: Rc<String>,
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
            Self::And(pos) => {
                if let Some(pos) = pos {
                    format!("'&&' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "&&".to_string()
                }
            }
            Self::Assign(pos) => {
                if let Some(pos) = pos {
                    format!("'=' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "=".to_string()
                }
            }
            Self::Asterisk(pos) => {
                if let Some(pos) = pos {
                    format!("'*' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "*".to_string()
                }
            }
            Self::Bang(pos) => {
                if let Some(pos) = pos {
                    format!("'!' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "!".to_string()
                }
            }
            Self::Bool(pos) => {
                if let Some(pos) = pos {
                    format!("'bool' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "bool".to_string()
                }
            }
            Self::Comma(pos) => {
                if let Some(pos) = pos {
                    format!("',' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    ",".to_string()
                }
            }
            Self::Diff(pos) => {
                if let Some(pos) = pos {
                    format!("'!=' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "!=".to_string()
                }
            }
            Self::EOF(pos) => {
                if let Some(pos) = pos {
                    format!("'EOF' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "EOF".to_string()
                }
            }
            Self::Else(pos) => {
                if let Some(pos) = pos {
                    format!("'else' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "else".to_string()
                }
            }
            Self::Eq(pos) => {
                if let Some(pos) = pos {
                    format!("'==' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "==".to_string()
                }
            }
            Self::Function(pos) => {
                if let Some(pos) = pos {
                    format!("'fn' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "fn".to_string()
                }
            }
            Self::Int(pos) => {
                if let Some(pos) = pos {
                    format!("'int' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "int".to_string()
                }
            }
            Self::LSqBracket(pos) => {
                if let Some(pos) = pos {
                    format!("'[' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "[".to_string()
                }
            }
            Self::Lbrace(pos) => {
                if let Some(pos) = pos {
                    format!("'{{' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "{{".to_string()
                }
            }
            Self::Let(pos) => {
                if let Some(pos) = pos {
                    format!("'let' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "let".to_string()
                }
            }
            Self::Lparen(pos) => {
                if let Some(pos) = pos {
                    format!("'(' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "(".to_string()
                }
            }
            Self::RSqBracket(pos) => {
                if let Some(pos) = pos {
                    format!("']' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "]".to_string()
                }
            }
            Self::Rbrace(pos) => {
                if let Some(pos) = pos {
                    format!("'}}' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "}}".to_string()
                }
            }
            Self::Return(pos) => {
                if let Some(pos) = pos {
                    format!("'ret' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "ret".to_string()
                }
            }
            Self::Rparen(pos) => {
                if let Some(pos) = pos {
                    format!("')' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    ")".to_string()
                }
            }
            Self::Semicolon(pos) => {
                if let Some(pos) = pos {
                    format!("';' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    ";".to_string()
                }
            }
            Self::Str(pos) => {
                if let Some(pos) = pos {
                    format!("'str' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "str".to_string()
                }
            }
            Self::True(pos) | Self::False(pos) => {
                if let Some(pos) = pos {
                    format!("'bool' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "bool".to_string()
                }
            }
            Self::Var(pos) => {
                if let Some(pos) = pos {
                    format!("'var' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "var".to_string()
                }
            }
            Self::While(pos) => {
                if let Some(pos) = pos {
                    format!("'while' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "while".to_string()
                }
            }
            Self::Number(val, pos) => match (val, pos) {
                (Some(_), Some(pos)) => {
                    format!("'number' in {}:{}:{}", pos.filename, pos.line, pos.position)
                }
                (_, _) => "number".to_string(),
            },
            Self::Illegal(val, pos) => match (val, pos) {
                (Some(val), Some(pos)) => {
                    format!(
                        "'illegal ({})' in {}:{}:{}",
                        val, pos.filename, pos.line, pos.position
                    )
                }
                (_, _) => "illegal".to_string(),
            },
            Self::Ident(val, pos) => match (val, pos) {
                (Some(val), Some(pos)) => {
                    format!(
                        "'identifier ({})' in {}:{}:{}",
                        val, pos.filename, pos.line, pos.position
                    )
                }
                (_, _) => "identifier".to_string(),
            },
            Self::String(val, pos) => match (val, pos) {
                (Some(val), Some(pos)) => {
                    format!(
                        "'String' ({}) in {}:{}:{}",
                        val, pos.filename, pos.line, pos.position
                    )
                }
                (_, _) => "String".to_string(),
            },
            _ => String::from("another thing, found it"),
        };
        write!(f, "{}", str)
    }
}
