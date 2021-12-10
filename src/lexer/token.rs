use std::{fmt::Display, hash::Hash, rc::Rc};

use crate::ast::types::Type;

#[derive(Debug)]
pub enum Token {
    Illegal(Option<Rc<String>>, Option<Location>),
    EOF(Option<Location>),
    Ident(Option<Rc<String>>, Option<Location>),
    Number(Option<Rc<String>>, Option<Location>),
    Comma(Option<Location>),
    Semicolon(Option<Location>),
    LParen(Option<Location>),
    RParen(Option<Location>),
    LSqBracket(Option<Location>),
    RSqBracket(Option<Location>),
    LBrace(Option<Location>),
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
    ShiftLeft(Option<Location>),
    ShiftRight(Option<Location>),
    BitWiseAnd(Option<Location>),
    BitWiseOr(Option<Location>),
    Xor(Option<Location>),
    Array(Option<Location>),
    Mod(Option<Location>),
}

#[derive(PartialEq, Debug, Clone, Hash)]
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
    pub(super) fn get_keyword_token(
        ident: &str,
        location: Option<Location>,
    ) -> Result<Token, String> {
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
            "array" => Ok(Token::Array(location)),
            _ => Err(String::from("Not a keyword")),
        }
    }

    pub fn literal(&self) -> String {
        match self {
            Self::Minus(_) => "-".to_string(),
            Self::Plus(_) => "+".to_string(),
            Self::Bang(_) => "!".to_string(),
            Self::Eq(_) => "==".to_string(),
            Self::Diff(_) => "!=".to_string(),
            Self::Gt(_) => ">".to_string(),
            Self::Lt(_) => "<".to_string(),
            Self::Gte(_) => ">=".to_string(),
            Self::Lte(_) => "<=".to_string(),
            Self::Slash(_) => "/".to_string(),
            Self::Asterisk(_) => "*".to_string(),
            Self::Or(_) => "||".to_string(),
            Self::And(_) => "&&".to_string(),
            Self::Assign(_) => "=".to_string(),
            Self::Array(_) => "array".to_string(),
            Self::ShiftLeft(_) => "<<".to_string(),
            Self::ShiftRight(_) => ">>".to_string(),
            Self::Xor(_) => "^".to_string(),
            Self::BitWiseAnd(_) => "&".to_string(),
            Self::BitWiseOr(_) => "|".to_string(),
            Self::Mod(_) => "%".to_string(),
            _ => "unknown".to_string(),
        }
    }

    pub fn to_type(&self) -> Type {
        match self {
            Self::Int(_)
            | Self::Mod(_)
            | Self::Plus(_)
            | Self::Minus(_)
            | Self::BitWiseOr(_)
            | Self::ShiftLeft(_)
            | Self::BitWiseAnd(_)
            | Self::ShiftRight(_)
            | Self::Xor(_) => Type::Int,
            Self::Array(_) => Type::Array,
            Self::Bool(_)
            | Self::Lt(_)
            | Self::Gt(_)
            | Self::Lte(_)
            | Self::Gte(_)
            | Self::Diff(_)
            | Self::And(_)
            | Self::Or(_)
            | Self::Eq(_) => Type::Bool,
            Self::Str(_) => Type::String,
            Self::Function(_) => Type::Function,
            _ => Type::Unknown,
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
            Self::LBrace(pos) => {
                if let Some(pos) = pos {
                    format!("'{{' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "{".to_string()
                }
            }
            Self::Let(pos) => {
                if let Some(pos) = pos {
                    format!("'let' in {}:{}:{}", pos.filename, pos.line, pos.position)
                } else {
                    "let".to_string()
                }
            }
            Self::LParen(pos) => {
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
            Self::RParen(pos) => {
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
            Self::If(pos) => match pos {
                Some(pos) => {
                    format!("'if'in {}:{}:{}", pos.filename, pos.line, pos.position)
                }
                _ => "if".to_string(),
            },
            Self::Or(pos) => match pos {
                Some(pos) => {
                    format!("'||'in {}:{}:{}", pos.filename, pos.line, pos.position)
                }
                _ => "if".to_string(),
            },
            Self::Array(pos) => match pos {
                Some(pos) => {
                    format!("'array'in {}:{}:{}", pos.filename, pos.line, pos.position)
                }
                _ => "if".to_string(),
            },
            Self::ShiftLeft(pos) => match pos {
                Some(pos) => {
                    format!("'<<'in {}:{}:{}", pos.filename, pos.line, pos.position)
                }
                _ => "if".to_string(),
            },
            Self::ShiftRight(pos) => match pos {
                Some(pos) => {
                    format!("'>>'in {}:{}:{}", pos.filename, pos.line, pos.position)
                }
                _ => "if".to_string(),
            },
            Self::BitWiseAnd(pos) => match pos {
                Some(pos) => {
                    format!("'&'in {}:{}:{}", pos.filename, pos.line, pos.position)
                }
                _ => "if".to_string(),
            },
            Self::BitWiseOr(pos) => match pos {
                Some(pos) => {
                    format!("'|'in {}:{}:{}", pos.filename, pos.line, pos.position)
                }
                _ => "if".to_string(),
            },
            Self::Xor(pos) => match pos {
                Some(pos) => {
                    format!("'^'in {}:{}:{}", pos.filename, pos.line, pos.position)
                }
                _ => "if".to_string(),
            },
            Self::Mod(pos) => match pos {
                Some(pos) => {
                    format!("'%'in {}:{}:{}", pos.filename, pos.line, pos.position)
                }
                _ => "if".to_string(),
            },
            _ => String::from("another thing, found it"),
        };
        write!(f, "{}", str)
    }
}

impl Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl std::cmp::Eq for Token {
    fn assert_receiver_is_total_eq(&self) {}
}
