use crate::ast::{expression::Expression, node::Node};

pub struct BoolEx {
    value: bool
}

impl BoolEx {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

impl Node for BoolEx {
    fn literal(&self) -> String {
        self.value.to_string()
    }
}

impl Expression for BoolEx {
    fn expression(&self) {
        todo!()
    }
}

