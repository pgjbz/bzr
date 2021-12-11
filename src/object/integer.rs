use std::{any::Any, fmt::Display};

use crate::ast::types::Type;

use super::Object;

pub struct Integer {
    pub val: i64,
}

impl Integer {
    pub fn new(val: i64) -> Self {
        Self { val }
    }
}

impl Object for Integer {
    fn get_type(&self) -> Type {
        Type::Int
    }

    fn inspect(&self) -> String {
        self.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}
