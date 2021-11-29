use std::fmt::Display;

use crate::{
    ast::{
        expression::Expression, identifier::Identifier, node::Node, statement::Statement,
        types::Type,
    },
    lexer::token::Token,
};

pub struct Let {
    token: Token,
    typ: Type,
    name: Identifier,
    value: Box<dyn Expression>,
}

impl Display for Let {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "tok: {}, typ: {}, name: {}, val: {}",
            self.token, self.typ, self.name, self.value
        )
    }
}

impl Let {
    pub fn new(token: Token, typ: Type, name: Identifier, value: Box<dyn Expression>) -> Box<Self> {
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

impl Node for Let {
    fn literal(&self) -> Box<dyn Display> {
        Box::new(format!("let {} {} = {}", self.name, self.typ, self.value))
    }
}

impl Statement for Let {
    fn statement(&self) {
        todo!()
    }

    fn get_statement_token(&self) -> Token {
        Token::Let(None)
    }
}
