use std::fmt::Display;

pub enum Type {
    String,
    Int,
    Bool,
    Unknown,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Self::String => "str",
            Self::Int => "int",
            Self::Bool => "bool",
            Self::Unknown => "unk",
        };
        write!(f, "{}", val)
    }
}
