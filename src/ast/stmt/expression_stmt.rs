use std::{fmt::Display, rc::Rc};

use crate::{
    ast::{
        expression::{Expression, Node},
        statement::Statement,
        types::Type,
    },
    lexer::token::Token,
};

pub struct ExpressionStatement {
    pub expression: Option<Rc<dyn Expression>>,
    pub typ: Type,
    pub token: Rc<Token>,
}

impl ExpressionStatement {
    pub fn new(typ: Type, token: Rc<Token>) -> Self {
        Self {
            expression: None,
            typ,
            token,
        }
    }
}

impl Node for ExpressionStatement {
    fn literal(&self) -> Box<dyn std::fmt::Display> {
        Box::new(String::new())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for ExpressionStatement {
    fn expression(&self) {
        todo!()
    }

    fn get_type(&self) -> Type {
        self.typ
    }

    fn set_type(&mut self, typ: Type) {
        self.typ = typ
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
