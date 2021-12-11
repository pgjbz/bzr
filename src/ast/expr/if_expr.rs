use std::{fmt::Display, rc::Rc};

use crate::ast::{
    expression::{Expression, Node},
    stmt::block_stmt::BlockStatement,
};

pub struct IfExpr {
    pub condition: Rc<dyn Expression>,
    pub consequence: Option<Rc<BlockStatement>>,
    pub alternative: Option<Rc<BlockStatement>>,
    pub el_if: Option<Rc<dyn Expression>>,
}

impl IfExpr {
    pub fn new(condition: Rc<dyn Expression>) -> Self {
        Self {
            condition,
            consequence: None,
            alternative: None,
            el_if: None,
        }
    }
}

impl Node for IfExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for IfExpr {
    fn get_type(&self) -> crate::ast::types::Type {
        crate::ast::types::Type::Expression
    }
}

impl Display for IfExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        buffer.push_str("if ");
        buffer.push_str(&format!("{} ", self.condition));
        buffer.push_str(&if let Some(ref consequence) = self.consequence {
            consequence.to_string()
        } else {
            "".to_string()
        });
        if let Some(ref alternative) = self.alternative {
            buffer.push_str(&format!(" else {}", alternative))
        };
        write!(f, "{}", buffer)
    }
}