use std::fmt::Display;

pub enum ParseError {
    Eof,
    Message(String),
    Unknown,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Eof => "EOF",
            Self::Unknown => "Unknown",
            Self::Message(msg) => msg,
        };
        write!(f, "{}", message)
    }
}
