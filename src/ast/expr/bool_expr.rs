use std::fmt::Display;

use crate::ast::{expression::Expression, node::Node, types::Type};

pub struct BoolExpr {
    pub value: bool,
}

impl BoolExpr {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

impl Node for BoolExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for BoolExpr {
    fn get_type(&self) -> Type {
        Type::Bool
    }
}

impl Display for BoolExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
