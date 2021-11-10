use std::fmt::Display;

use crate::ast::{expression::Expression, node::Node};

pub struct IntEx {
    value: i64,
}

impl IntEx {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

impl Node for IntEx {
    fn literal(&self) -> Box<dyn Display> {
        Box::new(self.value)
    }
}

impl Expression for IntEx {
    fn expression(&self) {
        todo!()
    }
}
