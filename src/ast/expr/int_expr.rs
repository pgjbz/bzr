use std::fmt::Display;

use crate::ast::{expression::Expression, node::Node, types::Type};

pub struct IntExpr {
    pub value: i64,
}

impl IntExpr {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

impl Node for IntExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for IntExpr {
    fn get_type(&self) -> Type {
        Type::Int
    }
}

impl Display for IntExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
