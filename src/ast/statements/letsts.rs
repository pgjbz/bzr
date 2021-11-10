use std::fmt::Display;

use crate::{
    ast::{
        expression::Expression, identifier::Identifier, node::Node, statement::Statement,
        types::Type,
    },
    lexer::token::Token,
};

pub struct Let {
    pub token: Token,
    pub typ: Type,
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl Display for Let {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "tok: {}, typ: {}, name: {}, val: {}", 
        self.token,
        self.typ,
        self.name,
        self.value)
    }
}

impl Let {
    pub fn new(
        token: Token,
        typ: Type,
        name: Identifier,
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

impl Node for Let {
    fn literal(&self) -> String {
        "let".to_string()
    }
}

impl Statement for Let {
    fn statement(&self) {
        todo!()
    }
}
