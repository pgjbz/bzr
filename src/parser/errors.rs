use std::{fmt::Display, num::ParseIntError};

pub(super) enum ParseError {
    Eof,
    Message(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Eof => "EOF",
            Self::Message(msg) => msg,
        };
        write!(f, "{}", message)
    }
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        Self::Message(e.to_string())
    }
}
