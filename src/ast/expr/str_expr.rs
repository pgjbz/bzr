use std::fmt::Display;

use crate::ast::{expression::Expression, node::Node, types::Type};

pub struct StrExpr {
    value: String,
}

impl StrExpr {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Node for StrExpr {
    fn literal(&self) -> Box<dyn Display> {
        Box::new(self.value.to_string())
    }
}

impl Expression for StrExpr {
    fn expression(&self) {
        todo!()
    }

    fn get_type(&self) -> Type {
        Type::String
    }
}
