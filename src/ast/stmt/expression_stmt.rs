use std::{fmt::Display, rc::Rc};

use crate::{
    ast::{
        expression::{Expression, Node},
        types::Type, statement::Statement,
    },
    lexer::token::Token,
};

pub struct ExpressionStatement {
    pub expression: Option<Box<dyn Expression>>,
    pub typ: Type,
    pub token: Rc<Token>,
}

impl ExpressionStatement {
    pub fn new(typ: Type, token: Rc<Token>) -> Self {
        Self {
            expression: None,
            typ,
            token
        }
    }
}

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

impl Statement for ExpressionStatement {
    fn statement(&self) {
        todo!()
    }

    fn get_statement_token(&self) -> Rc<Token> {
        Rc::clone(&self.token)
    }
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut expr = String::new();
        if let Some(ref expression) = self.expression {
            expr.push_str(&expression.to_string());
        }
        write!(f, "{}", expr)
    }
}
