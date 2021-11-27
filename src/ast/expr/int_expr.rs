use std::fmt::Display;

use crate::ast::{expression::Expression, node::Node};

pub struct IntExpr {
    value: i64,
}

impl IntExpr {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

impl Node for IntExpr {
    fn literal(&self) -> Box<dyn Display> {
        Box::new(self.value)
    }
}

impl Expression for IntExpr {
    fn expression(&self) {
        todo!()
    }
}
