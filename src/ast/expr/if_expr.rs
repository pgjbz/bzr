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
    pub condition: Box<dyn Expression>,
    pub consequence: Option<BlockStatement>,
    pub alternative: Option<BlockStatement>,
}

impl IfExpr {
    pub fn new(token: Rc<Token>, condition: Box<dyn Expression>) -> Self {
        Self {
            token,
            condition,
            consequence: None,
            alternative: None,
        }
    }
}

impl Node for IfExpr {
    fn literal(&self) -> Box<dyn std::fmt::Display> {
        todo!()
    }
}

impl Display for IfExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        buffer.push_str("if ");
        buffer.push_str(&format!("{} ", self.condition.to_string()));
        buffer.push_str(&if let Some(ref consequence) = self.consequence {
            consequence.to_string()
        } else {
            "".to_string()
        });
        if let Some(ref alternative) = self.alternative {
            buffer.push_str(&format!(" else {}", alternative.to_string()))
        };
        write!(f, "{}", buffer)
    }
}

impl Expression for IfExpr {
    fn expression(&self) {
        todo!()
    }

    fn get_type(&self) -> crate::ast::types::Type {
        todo!()
    }
}
