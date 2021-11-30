use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum Type {
    String,
    Int,
    Bool,
    Function,
    Prefix,
    Unknown,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Self::String => "str",
            Self::Int => "int",
            Self::Bool => "bool",
            Self::Function => "function",
            Self::Prefix => "prefix",
            Self::Unknown => "unk",
        };
        write!(f, "{}", val)
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
