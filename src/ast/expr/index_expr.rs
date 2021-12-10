use std::{fmt::Display, rc::Rc};

use crate::{
    ast::{
        expression::{Expression, Node},
        types::Type,
    },
    lexer::token::Token,
};

pub struct IndexExpr {
    pub token: Rc<Token>,
    pub left: Rc<dyn Expression>,
    pub index: Rc<dyn Expression>,
}

impl IndexExpr {
    pub fn new(token: Rc<Token>, left: Rc<dyn Expression>, index: Rc<dyn Expression>) -> Self {
        Self { token, left, index }
    }
}

impl Node for IndexExpr {
    fn literal(&self) -> Box<dyn std::fmt::Display> {
        Box::new("[")
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for IndexExpr {
    fn expression(&self) {
        todo!()
    }

    fn get_type(&self) -> Type {
        Type::Index
    }
}

impl Display for IndexExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        buffer.push_str(&format!("({}[", self.left));
        buffer.push_str(&format!("{}])", self.index));
        write!(f, "{}", buffer)
    }
}
