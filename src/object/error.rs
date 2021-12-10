use std::{any::Any, fmt::Display};

use crate::ast::types::Type;

use super::Object;

pub struct Error {
    pub val: String,
}

impl Error {
    pub fn new(val: String) -> Self {
        Self { val }
    }
}

impl Object for Error {
    fn get_type(&self) -> Type {
        Type::String
    }

    fn inspect(&self) -> String {
        self.val.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}
