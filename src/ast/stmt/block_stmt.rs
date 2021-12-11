use std::{fmt::Display, rc::Rc};

use crate::ast::{expression::Node, statement::Statement};

pub struct BlockStatement {
    pub statements: Vec<Rc<dyn Statement>>,
}

impl BlockStatement {
    pub fn new() -> Self {
        Self {
            statements: Vec::with_capacity(10),
        }
    }

    pub fn push_stmt(&mut self, stmt: Rc<dyn Statement>) {
        self.statements.push(stmt)
    }
}

impl Node for BlockStatement {
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

impl Default for BlockStatement {
    fn default() -> Self {
        Self::new()
    }
}
