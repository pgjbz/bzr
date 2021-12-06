use std::{any::Any, fmt::Display};

use crate::ast::types::Type;

use super::Object;

pub struct Str {
    pub val: String,
}

impl Str {
    pub fn new(val: String) -> Self {
        Self { val }
    }
}

impl Object for Str {
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

impl Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}
