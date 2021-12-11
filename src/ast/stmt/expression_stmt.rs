use std::{fmt::Display, rc::Rc};

use crate::ast::{
    expression::{Expression, Node},
    statement::Statement,
    types::Type,
};

pub struct ExpressionStatement {
    pub expression: Option<Rc<dyn Expression>>,
    pub typ: Type,
}

impl ExpressionStatement {
    pub fn new(typ: Type) -> Self {
        Self {
            expression: None,
            typ,
        }
    }
}

impl Node for ExpressionStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for ExpressionStatement {
    fn get_type(&self) -> Type {
        self.typ
    }

    fn set_type(&mut self, typ: Type) {
        self.typ = typ
    }
}

impl Statement for ExpressionStatement {}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut expr = String::new();
        if let Some(ref expression) = self.expression {
            expr.push_str(&expression.to_string());
        }
        write!(f, "{}", expr)
    }
}
