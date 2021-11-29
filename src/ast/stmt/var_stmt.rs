use std::fmt::Display;
use std::rc::Rc;

use crate::{
    ast::{
        expression::Expression, node::Node, statement::Statement,
        types::Type,
    },
    lexer::token::Token,
};

pub struct Var {
    pub token: Rc<Token>,
    typ: Type,
    name: Box<dyn Expression>,
    value: Box<dyn Expression>,
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stmt = String::new();
        stmt.push_str(&format!("{} ", self.literal()));
        stmt.push_str(&format!("{} ", self.name));
        stmt.push_str(&format!("{} ", self.typ));
        stmt.push_str("= ");
        stmt.push_str(&self.value.to_string());
        stmt.push(';');
        write!(f, "{}", stmt)
    }
}

impl Var {
    pub fn new(
        token: Rc<Token>,
        typ: Type,
        name: Box<dyn Expression>,
        value: Box<dyn Expression>,
    ) -> Box<Self> {
        Box::new({
            Self {
                token,
                typ,
                name,
                value,
            }
        })
    }
}

impl Node for Var {
    fn literal(&self) -> Box<dyn Display> {
        Box::new("var".to_string())
    }
}

impl Statement for Var {
    fn statement(&self) {
        todo!()
    }

    fn get_statement_token(&self) -> Rc<Token> {
        Rc::clone(&self.token)
    }
}
