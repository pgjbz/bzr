use crate::lexer::token::Token;
use std::fmt::Display;
use std::rc::Rc;

use crate::ast::{expression::Expression, node::Node, types::Type};

pub struct ArrayExpr {
    pub value: Vec<Rc<dyn Expression>>,
    pub token: Rc<Token>,
}

impl ArrayExpr {
    pub fn new(value: Vec<Rc<dyn Expression>>, token: Rc<Token>) -> Self {
        Self { value, token }
    }
}

impl Node for ArrayExpr {
    fn literal(&self) -> Box<dyn Display> {
        let mut buffer = String::new();
        buffer.push('[');
        for (pos, val) in self.value.iter().enumerate() {
            if pos < 1 {
                buffer.push_str(&val.to_string())
            } else {
                buffer.push_str(&format!(", {}", val))
            }
        }
        buffer.push(']');
        Box::new(buffer)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for ArrayExpr {
    fn expression(&self) {
        todo!()
    }

    fn get_type(&self) -> Type {
        Type::Array
    }
}

impl Display for ArrayExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.literal())
    }
}
