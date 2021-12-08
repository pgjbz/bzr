use std::{fmt::Display, rc::Rc};

use crate::{
    ast::{expression::Node, statement::Statement},
    lexer::token::Token,
};

pub struct BlockStatement {
    pub token: Rc<Token>,
    pub statements: Vec<Rc<dyn Statement>>,
}

impl BlockStatement {
    pub fn new(token: Rc<Token>) -> Self {
        Self {
            token,
            statements: Vec::with_capacity(10),
        }
    }

    pub fn push_stmt(&mut self, stmt: Rc<dyn Statement>) {
        self.statements.push(stmt)
    }
}

impl Node for BlockStatement {
    fn literal(&self) -> Box<dyn std::fmt::Display> {
        Box::new(self.token.literal())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        for stmt in self.statements.iter() {
            buffer.push_str(&stmt.to_string())
        }
        write!(f, "{}", buffer)
    }
}
