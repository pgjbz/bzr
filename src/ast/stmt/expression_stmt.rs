use std::fmt::Display;

use crate::{
    ast::{
        expression::{Expression, Node},
        types::Type,
    },
    lexer::token::Token,
};

pub struct ExpressionStatement {
    pub expression: Option<Box<dyn Expression>>,
    pub typ: Type,
    pub token: Token,
}

impl ExpressionStatement {}

impl Node for ExpressionStatement {
    fn literal(&self) -> Box<dyn std::fmt::Display> {
        todo!()
    }
}

impl Expression for ExpressionStatement {
    fn expression(&self) {
        todo!()
    }

    fn get_type(&self) -> Type {
        self.typ
    }
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut expr = String::new();
        while let Some(ref expression) = self.expression {
            expr.push_str(&format!("{}", expression));
        }
        write!(f, "{}", expr)
    }
}
