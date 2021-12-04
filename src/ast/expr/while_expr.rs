use std::{fmt::Display, rc::Rc};

use crate::{
    ast::{
        expression::{Expression, Node},
        stmt::block_stmt::BlockStatement,
        types::Type,
    },
    lexer::token::Token,
};

pub struct WhileExpr {
    pub token: Rc<Token>,
    pub condition: Box<dyn Expression>,
    pub consequence: Option<Box<BlockStatement>>,
}

impl WhileExpr {
    pub fn new(token: Rc<Token>, condition: Box<dyn Expression>) -> Self {
        Self {
            token,
            condition,
            consequence: None,
        }
    }
}

impl Node for WhileExpr {
    fn literal(&self) -> Box<dyn std::fmt::Display> {
        Box::new("while".to_string())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Display for WhileExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        buffer.push_str(&format!("{} ", self.literal()));
        buffer.push_str(&format!("{} ", self.condition));
        buffer.push_str(&if let Some(ref consequence) = self.consequence {
            consequence.to_string()
        } else {
            "".to_string()
        });
        write!(f, "{}", buffer)
    }
}

impl Expression for WhileExpr {
    fn expression(&self) {
        todo!()
    }

    fn get_type(&self) -> Type {
        Type::Expression
    }
}
