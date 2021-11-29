use crate::ast::expression::Expression;
use crate::ast::node::Node;
use crate::ast::types::Type;
use std::{fmt::Display, rc::Rc};

pub struct Identifier {
    pub value: Option<Rc<String>>,
}

impl Identifier {
    pub fn new(value: Option<Rc<String>>) -> Self {
        Self { value }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.as_ref().unwrap())
    }
}

impl Node for Identifier {
    fn literal(&self) -> std::boxed::Box<dyn Display> {
        Box::new("identifier".to_string())
    }
}

impl Expression for Identifier {
    fn expression(&self) {
        todo!()
    }
    fn get_type(&self) -> Type {
        Type::Unknown
    }
}
