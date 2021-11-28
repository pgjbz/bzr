use std::fmt::Display;

use crate::ast::{expression::Expression, node::Node, types::Type};

pub struct BoolExpr {
    value: bool,
}

impl BoolExpr {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

impl Node for BoolExpr {
    fn literal(&self) -> Box<dyn Display> {
        Box::new(self.value)
    }
}

impl Expression for BoolExpr {
    fn expression(&self) {
        todo!()
    }

    fn get_type(&self) -> Type {
        Type::Bool
    }
}
