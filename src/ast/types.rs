use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum Type {
    Int,
    Bool,
    Array,
    Error,
    Index,
    Prefix,
    String,
    BuiltIn,
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
            Self::Index => "Index",
            Self::Array => "array",
            Self::Error => "error",
            Self::Unknown => "unk",
            Self::Prefix => "prefix",
            Self::Function => "function",
            Self::Expression => "expression",
            Self::BuiltIn => "built in function",
        };
        write!(f, "{}", val)
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
