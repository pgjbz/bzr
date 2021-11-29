use std::rc::Rc;
use crate::lexer::token::Token;
use std::fmt::Display;

use crate::ast::{expression::Expression, node::Node, types::Type};

pub struct IntExpr {
    pub value: i64,
    pub token: Rc<Token>
}

impl IntExpr {
    pub fn new(value: i64, token: Rc<Token>) -> Self {
        Self { 
            value,
            token
        }
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

    fn get_type(&self) -> Type {
        Type::Int
    }
}
