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
    fn literal(&self) -> String {
        self.value.to_string()
    }
}

impl Expression for IntEx {
    fn expression(&self) {
        todo!()
    }
}
