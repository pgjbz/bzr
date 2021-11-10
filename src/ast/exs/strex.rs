use std::fmt::Display;

use crate::ast::{expression::Expression, node::Node};

pub struct StrEx {
    value: String,
}

impl StrEx {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Node for StrEx {
    fn literal(&self) -> Box<dyn Display> {
        Box::new(format!("{}", self.value))
    }
}

impl Expression for StrEx {
    fn expression(&self) {
        todo!()
    }
}
