use std::fmt::Display;

use crate::ast::{expression::Expression, node::Node, types::Type};

pub struct StrExpr {
    pub value: String,
}

impl StrExpr {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Node for StrExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for StrExpr {
    fn get_type(&self) -> Type {
        Type::String
    }
}

impl Display for StrExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
