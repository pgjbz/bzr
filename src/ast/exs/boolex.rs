use std::fmt::Display;

use crate::ast::{expression::Expression, node::Node};

pub struct BoolEx {
    value: bool,
}

impl BoolEx {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

impl Node for BoolEx {
    fn literal(&self) -> Box<dyn Display> {
        Box::new(self.value)
    }
}

impl Expression for BoolEx {
    fn expression(&self) {
        todo!()
    }
}
