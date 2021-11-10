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
            Self::String => "String",
            Self::Int => "Int",
            Self::Bool => "Bool",
            Self::Unknown => "Unk"
        };
        write!(f, "{}", val)
    }
}
