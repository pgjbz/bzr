use std::{any::Any, fmt::Display};

use crate::ast::types::Type;

use super::Object;

pub struct Boolean {
    pub val: bool,
}

impl Boolean {
    pub fn new(val: bool) -> Self {
        Self { val }
    }
}

impl Object for Boolean {
    fn get_type(&self) -> Type {
        Type::Bool
    }

    fn inspect(&self) -> String {
        self.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}
