use std::fmt::Display;

use crate::{
    ast::{
        expression::Expression, identifier::Identifier, node::Node, statement::Statement,
        types::Type,
    },
    lexer::token::Token,
};

pub struct Var {
    token: Token,
    typ: Type,
    name: Identifier,
    value: Box<dyn Expression>,
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "tok: {}, typ: {}, name: {}, val: {}",
            self.token, self.typ, self.name, self.value
        )
    }
}

impl Var {
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

impl Node for Var {
    fn literal(&self) -> Box<dyn Display> {
        Box::new(format!("var {} {} = {}", self.name, self.typ, self.value))
    }
}

impl Statement for Var {
    fn statement(&self) {
        todo!()
    }

    fn get_statament_token(&self) -> Token {
        Token::Var(None)
    }
}
