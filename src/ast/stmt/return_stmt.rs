use std::fmt::Display;

use crate::{
    ast::{
        expression::{Expression, Node},
        statement::Statement,
    },
    lexer::token::Token,
};

pub struct Return {
    pub return_value: Option<Box<dyn Expression>>,
}

impl Return {
    pub fn new(return_value: Option<Box<dyn Expression>>) -> Self {
        Self { return_value }
    }
}

impl Node for Return {
    fn literal(&self) -> Box<dyn Display> {
        Box::new("ret".to_string())
    }
}

impl Statement for Return {
    fn statement(&self) {
        todo!()
    }

    fn get_statement_token(&self) -> crate::lexer::token::Token {
        Token::Return(None)
    }
}
