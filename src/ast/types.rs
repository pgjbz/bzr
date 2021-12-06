use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum Type {
    Int,
    Bool,
    Error,
    Prefix,
    String,
    Unknown,
    Function,
    Expression,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Self::Int => "int",
            Self::Bool => "bool",
            Self::String => "str",
            Self::Error => "Error",
            Self::Unknown => "unk",
            Self::Prefix => "prefix",
            Self::Function => "function",
            Self::Expression => "expression",
        };
        write!(f, "{}", val)
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
