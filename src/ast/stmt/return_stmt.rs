use std::{fmt::Display, rc::Rc};

use crate::{
    ast::{
        expression::{Expression, Node},
        statement::Statement,
    },
    lexer::token::Token,
};

pub struct Return {
    pub token: Rc<Token>,
    pub return_value: Option<Box<dyn Expression>>,
}

impl Return {
    pub fn new(return_value: Option<Box<dyn Expression>>, token: Rc<Token>) -> Self {
        Self { return_value, token }
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

    fn get_statement_token(&self) -> Rc<Token> {
        Rc::clone(&self.token)
    }
}

impl Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stmt = String::new();
        stmt.push_str(&format!("{} ", &self.literal()));
        if let Some(ref ret_val) = self.return_value {
            stmt.push_str(&format!("{} ", ret_val));
        }
        stmt.push(';');
        write!(f, "{}", stmt)
    }
}
