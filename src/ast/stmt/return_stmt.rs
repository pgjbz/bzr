use std::{fmt::Display, rc::Rc};

use crate::ast::{
    expression::{Expression, Node},
    statement::Statement,
};

pub struct Return {
    pub return_value: Option<Rc<dyn Expression>>,
}

impl Return {
    pub fn new(return_value: Option<Rc<dyn Expression>>) -> Self {
        Self { return_value }
    }
}

impl Node for Return {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Statement for Return {}

impl Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stmt = String::new();
        stmt.push_str("ret");
        if let Some(ref ret_val) = self.return_value {
            stmt.push_str(&format!("{} ", ret_val));
        }
        stmt.push(';');
        write!(f, "{}", stmt)
    }
}
