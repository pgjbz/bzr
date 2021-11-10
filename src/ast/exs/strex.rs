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
    fn literal(&self) -> String {
        self.value.clone()
    }
}

impl Expression for StrEx {
    fn expression(&self) {
        todo!()
    }
}
