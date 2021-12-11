use std::fmt::Display;
use std::rc::Rc;

use crate::ast::{expression::Expression, node::Node, statement::Statement, types::Type};

pub struct Let {
    typ: Type,
    pub name: Rc<dyn Expression>,
    pub value: Rc<dyn Expression>,
}

impl Let {
    pub fn new(typ: Type, name: Rc<dyn Expression>, value: Rc<dyn Expression>) -> Rc<Self> {
        Rc::new(Self { typ, name, value })
    }
}

impl Node for Let {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Statement for Let {}

impl Display for Let {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stmt = String::new();
        stmt.push_str("let ");
        stmt.push_str(&format!("{} ", self.name));
        stmt.push_str(&format!("{} ", self.typ));
        stmt.push_str("= ");
        stmt.push_str(&self.value.to_string());
        stmt.push(';');
        write!(f, "{}", stmt)
    }
}