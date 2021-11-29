use crate::lexer::token::Token;
use std::fmt::Display;
use std::rc::Rc;

use crate::ast::{expression::Expression, node::Node, types::Type};

pub struct StrExpr {
    pub value: String,
    pub token: Rc<Token>,
}

impl StrExpr {
    pub fn new(value: String, token: Rc<Token>) -> Self {
        Self { value, token }
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
