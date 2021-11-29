use crate::lexer::token::Token;
use std::fmt::Display;
use std::rc::Rc;

use crate::ast::{expression::Expression, node::Node, types::Type};

pub struct BoolExpr {
    pub value: bool,
    pub token: Rc<Token>,
}

impl BoolExpr {
    pub fn new(value: bool, token: Rc<Token>) -> Self {
        Self { value, token }
    }
}

impl Node for BoolExpr {
    fn literal(&self) -> Box<dyn Display> {
        Box::new(self.value)
    }
}

impl Expression for BoolExpr {
    fn expression(&self) {
        todo!()
    }

    fn get_type(&self) -> Type {
        Type::Bool
    }
}

impl Display for BoolExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.literal())
    }
}
