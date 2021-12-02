use std::fmt::Display;

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
