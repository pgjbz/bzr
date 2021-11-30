use std::fmt::Display;

pub enum ParseError {
    Eof,
    Message(String),
    Unknown(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Eof => "EOF",
            Self::Unknown(msg) => msg,
            Self::Message(msg) => msg,
        };
        write!(f, "{}", message)
    }
}
