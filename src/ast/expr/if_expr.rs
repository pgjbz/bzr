use std::{fmt::Display, rc::Rc};

use crate::{
    ast::{
        expression::{Expression, Node},
        stmt::block_stmt::BlockStatement,
    },
    lexer::token::Token,
};

pub struct IfExpr {
    pub token: Rc<Token>,
    pub condition: Rc<dyn Expression>,
    pub consequence: Option<Rc<BlockStatement>>,
    pub alternative: Option<Rc<BlockStatement>>,
    pub el_if: Option<Rc<dyn Expression>>,
}

impl IfExpr {
    pub fn new(token: Rc<Token>, condition: Rc<dyn Expression>) -> Self {
        Self {
            token,
            condition,
            consequence: None,
            alternative: None,
            el_if: None,
        }
    }
}

impl Node for IfExpr {
    fn literal(&self) -> Box<dyn std::fmt::Display> {
        Box::new("if".to_string())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Display for IfExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        buffer.push_str(&format!("{} ", self.literal()));
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

impl Expression for IfExpr {
    fn expression(&self) {
        todo!()
    }

    fn get_type(&self) -> crate::ast::types::Type {
        crate::ast::types::Type::Expression
    }
}
